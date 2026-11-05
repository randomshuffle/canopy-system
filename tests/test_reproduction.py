"""Regression checks for published data selection and reproduction safety."""
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
from benchmark_common import ROOT, require_writable_campaign_output
from reproduce import CONFIG, REFERENCE, compare_table, jobs, report


class ReproductionTests(unittest.TestCase):
    def test_plan_reproduces_completed_scope_and_each_recorded_sampling_policy(self):
        cfg = json.loads(CONFIG.read_text())
        plan = jobs(cfg, REFERENCE, Path('target/release'))
        self.assertEqual(len(plan), 78)
        self.assertEqual(sum(job['points'] for job in plan), 159)
        for job in plan:
            record = json.loads((Path(job['output']) / 'process.json').read_text())
            command = record['identity']['command']
            self.assertEqual(job['warmups'], int(command[command.index('--warmups') + 1]))
            self.assertEqual(job['repetitions'], int(command[command.index('--repetitions') + 1]))
            self.assertEqual(record['status'], 'complete')
            self.assertEqual(record['exit_code'], 0)
            if job['family'] == 'accumulation':
                self.assertEqual(job['log_input'], 24)
            elif job['threads'] == 1:
                self.assertLessEqual(job['log_input'], 26)

    def test_reference_sources_and_symlinks_cannot_be_output_destinations(self):
        for path in (ROOT, ROOT.parent, ROOT / 'results', REFERENCE,
                     REFERENCE / 'new-run', ROOT / 'src/new-run'):
            with self.subTest(path=path), self.assertRaises(ValueError):
                require_writable_campaign_output(path)
        with tempfile.TemporaryDirectory() as directory:
            alias = Path(directory) / 'reference'
            alias.symlink_to(REFERENCE, target_is_directory=True)
            with self.assertRaises(ValueError):
                require_writable_campaign_output(alias / 'new-run')
            require_writable_campaign_output(Path(directory) / 'fresh-run')
        require_writable_campaign_output(ROOT / 'results/reproduce/fresh-run')

    def test_all_published_fields_reconstruct_from_raw_samples(self):
        cfg = json.loads(CONFIG.read_text())
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            self.assertEqual(report(cfg, REFERENCE, Path('.'), output), 159)
            for name, count in (('snark_comparison.csv', 135), ('accumulation_comparison.csv', 24),
                                ('proof_sizes_and_hashes.csv', 75)):
                keys = ('log_input', 'input_oracles', 'scheme')
                if name != 'proof_sizes_and_hashes.csv':
                    keys += ('threads',)
                self.assertEqual(compare_table(output / name, REFERENCE / name, keys), count)


if __name__ == '__main__':
    unittest.main()
