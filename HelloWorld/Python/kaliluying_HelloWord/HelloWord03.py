import time


def main():
    # 初始化字符串
    hello_world = "Hello World"

    # 执行行数
    for i in range(len(hello_world)):
        # 从0到i
        for j in range(i + 1):
            # 打印前i个字符
            print(hello_world[j], end='')
            # 休眠
            time.sleep(0.05)
        # 换行
        print()


if __name__ == "__main__":
    main()
