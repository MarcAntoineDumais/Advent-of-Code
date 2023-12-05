import time
import day5

def main():
    start_time = time.time()
    
    day5.part2()

    elapsed = (time.time() - start_time) * 1000
    print(f"Execution time: {elapsed}ms")

if __name__ == "__main__":
    main()
