import java.util.Arrays;
import java.util.List;

public class main {

	public static void main(String[] args) {		

		extraTests();
		
		long streamStart = System.currentTimeMillis();
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.stream()).get().price.get().intValue());
		long streamEnd = System.currentTimeMillis();
		System.out.println((streamEnd-streamStart));

		long parallelStart = System.currentTimeMillis();
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.parallelStream()).get().price.get().intValue());
		long parallelEnd = System.currentTimeMillis();
		
		System.out.println(parallelEnd-parallelStart);
	
	}

	private static void extraTests(){
		System.out.println("AMZN Price: " + APIFinance.getPrice("AMZN").get().toString());
		System.out.println("ABCDEFG Price: " + APIFinance.getPrice("ABCDEFG").toString());
		System.out.println("Empty string Price: " + APIFinance.getPrice("").toString());

		List<String> testList = Arrays.asList("SHOP", "TSLA");
		System.out.println(PickShareFunctional.findHighPriced(testList.stream()).get().price.get().intValue());

		testList = Arrays.asList();
		System.out.println(PickShareFunctional.findHighPriced(testList.stream()).get().price.toString());
		
		testList = Arrays.asList("SHOP");
		System.out.println(PickShareFunctional.findHighPriced(testList.stream()).get().price.get().intValue());
		
		System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.stream()).get().price.get().intValue());
	}

}