import akioi_2048 as ak
from tqdm import tqdm
import time
import printer

b = ak.init()
for i in range(20):
    b, *_ = ak.step(b, i % 4)

printer.print_table(b)
start = time.time()
for _ in tqdm(range(1000000)):
    ak.step(b, 1)
end = time.time()

print(f"耗时：{end - start:.6f} 秒")
