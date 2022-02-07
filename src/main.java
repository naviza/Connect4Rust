public class main {

	public static void main(String[] args) {		
		
		long streamStart = System.currentTimeMillis();
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.stream()).get().price.get().intValue());
		long streamEnd = System.currentTimeMillis();
		System.out.println((streamEnd-streamStart));

		long parallelStart = System.currentTimeMillis();
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.parallelStream()).get().price.get().intValue());
		long parallelEnd = System.currentTimeMillis();
		
		System.out.println(parallelEnd-parallelStart);
	
	}

}