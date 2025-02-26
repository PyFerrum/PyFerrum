import pyferrum

def read_csv(file_path):
    """Rust에서 제공하는 CSV 파싱 기능을 호출"""
    return pyferrum.read_csv(file_path)
