// https://circleci.com/blog/api-performance-testing-with-k6/
import http from 'k6/http';
import { check, group } from 'k6';

export let options = {
   stages: [
       { duration: '0.5m', target: 3 }, // simulate ramp-up of traffic from 1 to 3 virtual users over 0.5 minutes.
       { duration: '0.5m', target: 4}, // stay at 4 virtual users for 0.5 minutes
       { duration: '0.5m', target: 0 }, // ramp-down to 0 users
     ]
};

export default function () {
   group('API check', () => {
       const response = http.get('http://localhost:7878/hello');
       check(response, {
           "status code should be 200": res => res.status === 200,
       });
   });
}

