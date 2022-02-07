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

//    private static void extraTests(){
//        System.out.println("AMZN Price: " + APIFinance.getPrice("AMZN").get().toString());
//        System.out.println("ABCDEFG Price: " + APIFinance.getPrice("ABCDEFG").toString());
//        System.out.println("Empty string Price: " + APIFinance.getPrice("").toString());
//
//        List<String> testList = Arrays.asList("SHOP", "TSLA");
//        System.out.println("{SHOP, TSLA}: " +
//            PickShareFunctional.findHighPriced(testList.stream()).get().price.get().toString());
//
//        System.out.println("Stream time(ms) = " + (streamFinish-streamStart));
//        System.out.println("Parallel Stream time(ms) = " + (parallelFinish-parallelStart));
//    }
}
