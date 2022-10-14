import http from 'k6/http';

export default function () {
  http.post('http://localhost:3091/api/session/auth');
}
