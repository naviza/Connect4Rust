import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class main {

	public static void main(String[] args) {		
		
		long streamStart = System.currentTimeMillis();
		System.out.println("Highest share price is " + PickShareFunctional.findHighPriced(Shares.symbols.stream()).get());
		long streamEnd = System.currentTimeMillis();
		System.out.println("Time taken (ms) = " + (streamEnd-streamStart));

		long parallelStart = System.currentTimeMillis();
		System.out.println("Highest share price is " + PickShareFunctional.findHighPriced(Shares.symbols.parallelStream()).get());
		long parallelEnd = System.currentTimeMillis();
		
		System.out.println("Time taken (ms) = " + (parallelEnd-parallelStart));

		// extraTests();
	
	}

	private static void extraTests(){
		System.out.println("AMZN Price: " + APIFinance.getPrice("AMZN").get().toString());
		System.out.println("ABCDEFG Price: " + APIFinance.getPrice("ABCDEFG").toString());
		System.out.println("Empty string Price: " + APIFinance.getPrice("").toString());

		List<String> testList = Arrays.asList("SHOP", "TSLA");
		System.out.println("{SHOP, TSLA}: " +
			PickShareFunctional.findHighPriced(testList.stream()).get().price.get().toString());

		testList = Arrays.asList();
		System.out.println("{}: " +
			PickShareFunctional.findHighPriced(testList.stream()).toString()
			);
		
		testList = Arrays.asList("SHOP");
		System.out.println("{SHOP}: " +
			PickShareFunctional.findHighPriced(testList.stream()).get().price.toString()
			);
		
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.stream()).get().price.get().intValue());
		List<String> noAmazon = new ArrayList<String>(Shares.symbols); noAmazon.remove("AMZN");
		System.out.println(
			PickShareFunctional.findHighPriced(noAmazon.stream()).get().price.get().intValue()
			);
	}
}