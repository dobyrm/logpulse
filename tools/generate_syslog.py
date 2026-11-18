import os
import random
import time
from datetime import datetime, timezone

SERVICES = [
    ("sshd", [
        "Accepted publickey for root from 192.168.1.50 port 52210 ssh2",
        "Failed password for invalid user admin from 203.0.113.42 port 38412",
        "ERROR Disconnecting: Too many authentication failures for root",
        "Connection closed by 10.0.0.15 port 44100"
    ]),
    ("nginx", [
        "192.168.1.10 - - [13/Nov/2026:10:45:30 +0200] \"GET /api/v1/resource HTTP/1.1\" 200 1024",
        "ERROR 502 Bad Gateway upstream server disconnected while reading response",
        "10.0.0.5 - - [13/Nov/2026:10:45:32 +0200] \"GET /healthz HTTP/1.1\" 200 45",
        "192.168.1.15 - - [13/Nov/2026:10:45:33 +0200] \"GET /admin/db HTTP/1.1\" 500 512"
    ]),
    ("kernel", [
        "Out of memory: Kill process 1420 (postgres) score 850 or sacrifice child",
        "ERROR TCP: drop SYN to 192.168.1.100:443 due to SYN flood",
        "EXT4-fs (sda1): mounted filesystem with ordered data mode",
        "CPU 3: Core thermal trip detected, reducing frequency"
    ]),
    ("postgres", [
        "LOG: database system was shut down at 2026-11-13 10:00:00 EET",
        "FATAL: password authentication failed for user \"app_user\"",
        "ERROR: deadlock detected in process 2310 waiting for ExclusiveLock",
        "LOG: automatic vacuum of table \"public.transactions\": found 12 dead rows"
    ])
]

HOSTS = ["server-us-east-01", "node-worker-04", "db-cluster-primary", "edge-proxy-02"]

def generate_syslog_file(filename="inputs/syslog.log", target_size_mb=10):
    os.makedirs("inputs", exist_ok=True)
    print(f"Generating standard BSD Syslog file (RFC 3164) {filename} (~{target_size_mb} MB)...")
    
    target_size_bytes = target_size_mb * 1024 * 1024
    written_bytes = 0
    start_time = time.time()
    
    base_timestamp = 1794570000 
    
    with open(filename, "w", encoding="utf-8") as f:
        i = 0
        while written_bytes < target_size_bytes:
            base_timestamp += random.randint(30, 150)
            
            dt = datetime.fromtimestamp(base_timestamp, tz=timezone.utc)
            timestamp_str = dt.strftime("%b %d %H:%M:%S")
            
            host = random.choice(HOSTS)
            service_name, messages = random.choice(SERVICES)
            pid = random.randint(100, 9999)
            
            # Кожен 5-й запис гарантовано робимо з помилкою ERROR для детекції інциденту
            if i % 5 == 0:
                error_msgs = [m for m in messages if "ERROR" in m or "FATAL" in m]
                message = error_msgs[0] if error_msgs else "ERROR: unexpected crash"
            else:
                normal_msgs = [m for m in messages if "ERROR" not in m and "FATAL" not in m]
                message = normal_msgs[0] if normal_msgs else messages[0]
            
            line = f"{timestamp_str} {host} {service_name}[{pid}]: {message}\n"
            
            f.write(line)
            written_bytes += len(line.encode('utf-8'))
            i += 1
            
    elapsed = time.time() - start_time
    print(f"Done! RFC 3164 Syslog created in {elapsed:.2f} sec. Path: {filename}")

if __name__ == "__main__":
    generate_syslog_file()
