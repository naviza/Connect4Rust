public class main {

	public static void main(String[] args) {
		// TODO Auto-generated method stub
		
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.stream()).get().price.get().intValue());
		

		long streamStart = System.currentTimeMillis();
		PickShareFunctional.findHighPriced(Shares.symbols.stream());
		long streamEnd = System.currentTimeMillis();
		System.out.println((streamEnd-streamStart));

		long parallelStart = System.currentTimeMillis();
		PickShareFunctional.findHighPriced(Shares.symbols.parallelStream());
		long parallelEnd = System.currentTimeMillis();

		System.out.println(parallelEnd-parallelStart);
	
	}

}