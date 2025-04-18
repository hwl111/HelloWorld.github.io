public class HelloWorld03 {
    public static void main(String[] args) throws InterruptedException {
        //初始化数组
        char[] helloWorld = "Hello World".toCharArray();
        //执行行数
        for (int i = 0; i < helloWorld.length; i++) {
            //从 0 到 i
            for (int j = 0; j <= i; j++) {
                //打印前 i 个字符
                System.out.print(helloWorld[j]);
                //休眠
                Thread.sleep(50);
            }
            //换行
            System.out.println();
        }
    }
}
