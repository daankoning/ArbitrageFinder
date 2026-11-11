import io
import json
import unittest
from contextlib import redirect_stdout
from unittest.mock import Mock, patch
import main
from src import logic

class RegressionTests(unittest.TestCase):
    def test_timeout_is_retained_and_printable(self):
        for call in (lambda:logic.get_sports('test'),lambda:logic.get_data('test','soccer_test')):
            with patch.object(logic.requests,'get',side_effect=logic.requests.exceptions.Timeout):
                with self.assertRaises(logic.TimeoutException) as caught: call()
                self.assertIn('Timed out',str(caught.exception))
    def test_http_error_does_not_require_json(self):
        response=Mock(status_code=503)
        response.json.side_effect=ValueError('not JSON')
        with self.assertRaises(logic.APIException) as caught: logic.handle_faulty_response(response)
        self.assertIn('HTTP 503',str(caught.exception))
    def test_h2h_request_retains_decimal_odds_and_timeout(self):
        response=Mock(status_code=200);response.json.return_value=[]
        with patch.object(logic.requests,'get',return_value=response) as request:
            self.assertEqual(logic.get_data('test','soccer_test'),[])
        self.assertEqual(request.call_args.kwargs['timeout'],30)
        self.assertEqual(request.call_args.kwargs['params']['markets'],'h2h')
        self.assertEqual(request.call_args.kwargs['params']['oddsFormat'],'decimal')
    def test_cutoff_is_return_not_probability_gap(self):
        candidate={'total_implied_odds':.905}
        with patch.object(logic,'get_sports',return_value=[]),patch.object(logic,'process_data',return_value=[candidate]):
            self.assertEqual(list(logic.get_arbitrage_opportunities('test','eu',.1)),[candidate])
    def test_unformatted_output_is_json_with_separate_equal_odds(self):
        candidate={'match_name':'A v B','league':'test','total_implied_odds':2/2.1,'best_outcome_odds':{'A':('One',2.1),'B':('Two',2.1)}}
        output=io.StringIO()
        with patch('sys.argv',['main.py','--unformatted']),patch.object(main,'load_dotenv'),patch.object(main,'get_arbitrage_opportunities',return_value=[candidate]),redirect_stdout(output):
            main.main()
        result=json.loads(output.getvalue())[0]['analysis']
        self.assertEqual([x['stake'] for x in result['legs']],[50,50])
        self.assertAlmostEqual(result['worstNet'],5)
    def test_invalid_budget_rejected_before_api(self):
        with patch('sys.argv',['main.py','--budget','nan']),patch.object(main,'load_dotenv'),patch.object(main,'get_arbitrage_opportunities') as fetch:
            with self.assertRaises(SystemExit) as caught: main.main()
            self.assertEqual(caught.exception.code,2)
            fetch.assert_not_called()
if __name__=='__main__': unittest.main()
