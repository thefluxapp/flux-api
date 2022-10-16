import http from 'k6/http';

export const options = {
  vus: 100,
  duration: '10s',
};

export default function () {
  // http.post('http://localhost:3091/api/session/auth');
  http.get('http://localhost:3091/api/session');
}
