import requests

BASE = "http://localhost:8080"

def test_upload_endpoint_exists():
    # This test intentionally expects a 200 or 4xx until the server is implemented.
    try:
        r = requests.options(f"{BASE}/api/upload")
    except Exception:
        assert False, "Server not running or endpoint unreachable"
    assert r.status_code in (200, 401, 400, 405), f"Unexpected status: {r.status_code}"
