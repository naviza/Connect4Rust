public class main {

	public static void main(String[] args) {
		// TODO Auto-generated method stub
		System.out.printf("%d", PickShareFunctional.findHighPriced(Shares.symbols.stream()).get().price.get().intValue());
	}

}