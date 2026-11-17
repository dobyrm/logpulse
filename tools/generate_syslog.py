import random
import time

SERVICES = [
    ("sshd", [
        "Accepted publickey for root from 192.168.1.50 port 52210 ssh2",
        "Failed password for invalid user admin from 203.0.113.42 port 38412",
        "Disconnecting: Too many authentication failures for root",
        "Connection closed by 10.0.0.15 port 44100"
    ]),
    ("nginx", [
        "192.168.1.10 - - [06/Nov/2026:16:45:30 +0200] \"GET /api/v1/resource HTTP/1.1\" 200 1024",
        "203.0.113.88 - - [06/Nov/2026:16:45:31 +0200] \"POST /login HTTP/1.1\" 401 184",
        "10.0.0.5 - - [06/Nov/2026:16:45:32 +0200] \"GET /healthz HTTP/1.1\" 200 45",
        "192.168.1.15 - - [06/Nov/2026:16:45:33 +0200] \"GET /admin/db HTTP/1.1\" 500 512"
    ]),
    ("kernel", [
        "Out of memory: Kill process 1420 (postgres) score 850 or sacrifice child",
        "EXT4-fs (sda1): mounted filesystem with ordered data mode",
        "CPU 3: Core thermal trip detected, reducing frequency",
        "TCP: drop SYN to 192.168.1.100:443 due to SYN flood"
    ]),
    ("postgres", [
        "LOG: database system was shut down at 2026-11-06 14:00:00 EET",
        "FATAL: password authentication failed for user \"app_user\"",
        "ERROR: deadlock detected in process 2310 waiting for ExclusiveLock",
        "LOG: automatic vacuum of table \"public.transactions\": found 12 dead rows"
    ])
]

HOSTS = ["server-us-east-01", "node-worker-04", "db-cluster-primary", "edge-proxy-02"]

def generate_syslog_file(filename="inputs/syslog.log", target_size_mb=10):
    print(f"Generating standard BSD Syslog file (RFC 3164) {filename} (~{target_size_mb} MB)...")
    
    target_size_bytes = target_size_mb * 1024 * 1024
    written_bytes = 0
    start_time = time.time()
    
    with open(filename, "w", encoding="utf-8") as f:
        base_timestamp = int(time.time()) - 86400 * 7
        
        while written_bytes < target_size_bytes:
            base_timestamp += random.randint(1, 4)
            t_struct = time.localtime(base_timestamp)
            
            timestamp_str = time.strftime("%b %e %H:%M:%S", t_struct)
            
            host = random.choice(HOSTS)
            service_name, messages = random.choice(SERVICES)
            pid = random.randint(100, 9999)
            message = random.choice(messages)
            
            line = f"{timestamp_str} {host} {service_name}[{pid}]: {message}\n"
            
            f.write(line)
            written_bytes += len(line.encode('utf-8'))
            
    elapsed = time.time() - start_time
    print(f"Done! RFC 3164 Syslog created in {elapsed:.2f} sec. Path: {filename}")

if __name__ == "__main__":
    generate_syslog_file("inputs/syslog.log", target_size_mb=10)
