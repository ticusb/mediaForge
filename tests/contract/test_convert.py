import requests

BASE = "http://localhost:8080"

def test_convert_endpoint_exists():
    try:
        r = requests.options(f"{BASE}/api/convert")
    except Exception:
        assert False, "Server not running or endpoint unreachable"
    assert r.status_code in (200, 401, 400, 405), f"Unexpected status: {r.status_code}"
