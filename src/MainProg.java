
public class MainProg {

	public static void main(String[] args) {
		System.out.printf("%d", PickShareFunctional.findHighPriced(Shares.symbols.stream()).price.intValue());
		System.out.println("Hello World, Java app");
	}

}
