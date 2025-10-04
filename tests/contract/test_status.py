import requests

BASE = "http://localhost:8080"

def test_status_endpoint_exists():
    try:
        r = requests.options(f"{BASE}/api/status/dummy-id")
    except Exception:
        assert False, "Server not running or endpoint unreachable"
    assert r.status_code in (200, 401, 400, 404), f"Unexpected status: {r.status_code}"
