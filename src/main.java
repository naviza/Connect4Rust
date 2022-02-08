import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.TimeUnit;

public class main {

    public static void main(String[] args) {        
        try {
        	System.out.println("Resetting API time limit...");
            TimeUnit.SECONDS.sleep(60);
            APIFinance.reset_counter();
            System.out.println("API time limit reset");
        	
            long streamStart = System.currentTimeMillis();
            System.out.println("Highest share price is " + PickShareFunctional.findHighPriced(Shares.symbols.stream()).get());
            long streamEnd = System.currentTimeMillis();
            System.out.println("Time taken (ms) = " + (streamEnd-streamStart));
            
            System.out.println("Resetting API time limit...");
            TimeUnit.SECONDS.sleep(60);
            APIFinance.reset_counter();
            System.out.println("API time limit reset");
            
            long parallelStart = System.currentTimeMillis();
            System.out.println("Highest share price is " + PickShareFunctional.findHighPriced(Shares.symbols.parallelStream()).get());
            long parallelEnd = System.currentTimeMillis();
            System.out.println("Time taken (ms) = " + (parallelEnd-parallelStart));
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        // extraTests();
    }

	private static void extraTests(){
        // Tests for APIFinance.getPrice
		System.out.println("AMZN Price: " + APIFinance.getPrice("AMZN").toString());
		System.out.println("ABCDEFG Price: " + APIFinance.getPrice("ABCDEFG").toString());
		System.out.println("Empty string Price: " + APIFinance.getPrice("").toString());


        try {
            // Small Tests for PickShareFunctional.findHighPriced
            TimeUnit.SECONDS.sleep(60); // Have to reset the timer
            APIFinance.reset_counter(); // also reset the timer
            List<String> testList = Arrays.asList("SHOP", "TSLA");
            System.out.println("{SHOP, TSLA}: " +
                PickShareFunctional.findHighPriced(testList.stream()).toString()
                );

            testList = Arrays.asList();
            System.out.println("{}: " +
                PickShareFunctional.findHighPriced(testList.stream()).toString()
                );
            
            testList = Arrays.asList("SHOP");
            System.out.println("{SHOP}: " +
                PickShareFunctional.findHighPriced(testList.stream()).toString()
                );
            
            TimeUnit.SECONDS.sleep(60);
            APIFinance.reset_counter();
            // Big Tests for PickShareFunctional.findHighPriced
            System.out.println(PickShareFunctional.findHighPriced(Shares.symbols.stream()));
            List<String> noAmazon = new ArrayList<String>(Shares.symbols); noAmazon.remove("AMZN");
            System.out.println(
                PickShareFunctional.findHighPriced(noAmazon.stream())
                );
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
	}
}
