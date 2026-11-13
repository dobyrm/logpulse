import random
import time

LOG_LEVELS = ["INFO", "INFO", "INFO", "WARN", "ERROR", "DEBUG"]

MESSAGES = {
    "INFO": [
        "User session started successfully for user_id={}",
        "Cache hit for key /api/v1/resource/{}",
        "Database connection pool acquired connection from idle pool",
        "HTTP request GET /healthz completed with status 200 in {}ms"
    ],
    "WARN": [
        "High memory usage detected: {}% utilized",
        "Rate limit approaching for IP 192.168.1.{}",
        "Deprecated API endpoint /v0/old-route accessed",
        "Request timeout warning on upstream service after {}ms"
    ],
    "ERROR": [
        "Failed to connect to PostgreSQL cluster at 10.0.0.{}: Connection refused",
        "NullPointerException in AuthModule at line {}",
        "Disk write error on partition /var/log/{}",
        "Transaction rolled back due to deadlock in table users_v{}"
    ],
    "DEBUG": [
        "Executing query: SELECT * FROM transactions WHERE id = {}",
        "Payload received: size={} bytes",
        "Worker thread #{} picked up job from queue",
        "Token validation successful, claims={}"
    ]
}

def generate_ip():
    return f"{random.randint(1, 255)}.{random.randint(0, 255)}.{random.randint(0, 255)}.{random.randint(1, 255)}"

def generate_log_file(filename="inputs/system.log", target_size_mb=100):
    print(f"File generation {filename} in size ~{target_size_mb} MB...")
    
    target_size_bytes = target_size_mb * 1024 * 1024
    written_bytes = 0
    start_time = time.time()
    
    with open(filename, "w", encoding="utf-8") as f:
        base_timestamp = int(time.time()) - 86400 * 7
        
        while written_bytes < target_size_bytes:
            base_timestamp += random.randint(1, 3)
            time_str = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(base_timestamp))
            
            level = random.choice(LOG_LEVELS)
            msg_template = random.choice(MESSAGES[level])
            msg = msg_template.format(random.randint(1, 10000))
            ip = generate_ip()
            
            line = f"[{time_str}] [{level}] [{ip}] {msg}\n"
            
            f.write(line)
            written_bytes += len(line.encode('utf-8'))
            
    elapsed = time.time() - start_time
    print(f"Done! File successfully created by {elapsed:.2f} Sec. Path: {filename}")

if __name__ == "__main__":
    generate_log_file("inputs/system.log", target_size_mb=100)
