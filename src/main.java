public class main {

	public static void main(String[] args) {
		// TODO Auto-generated method stub
		long streamStart = System.currentTimeMillis();
		System.out.printf("%d\n", PickShareFunctional.findHighPriced(Shares.symbols.stream()).price.intValue());
		long streamFinish = System.currentTimeMillis();
		long parallelStart = System.currentTimeMillis();
		System.out.printf("%d\n", PickShareFunctional.findHighPriced(Shares.symbols.parallelStream()).price.intValue());
		long parallelFinish = System.currentTimeMillis();

		System.out.println("Stream time(ms) = " + (streamFinish-streamStart));
		System.out.println("Parallel Stream time(ms) = " + (parallelFinish-parallelStart));
	}
}
