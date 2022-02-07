import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.math.BigDecimal;
import java.net.URL;
import java.net.URLConnection;
import java.util.Optional;
import java.util.concurrent.TimeUnit;

public class APIFinance {
	private static final String BASE_URL = "https://www.alphavantage.co/query?";
	private final static String apiKey = "0BEBA6Z10REBY4Z5";
	private volatile static int counter = 0;  // Counter of number of API calls that shares between threads

	public static Optional<BigDecimal> getPrice(final String symbol) {
		try {
			// Checks that only 5 API calls are made per minute
			synchronized (APIFinance.class) {
				if (counter > 5) {
					TimeUnit.SECONDS.sleep(60);  // Sleep for a minute to reset API
					counter = 0;  // Reset counter
					// System.out.println("I slept");
				}
				counter++;
			}
			URL url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
			// Instantiate the objects to obtain values
			URLConnection connection = url.openConnection();
			InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
			BufferedReader bufferedReader = new BufferedReader(inputStream);
			// Process the buffered stream of lines
			return (bufferedReader.lines()
				.filter(line -> line.contains("price")) // take the line that indicates "price"
				// map the value to an Optional<BigDecimal> if it exists
				.findFirst().map(value -> Optional.of(new BigDecimal(value.split("\"")[3].trim())))
				.orElse(Optional.empty())); // return Empty if the "price" line does not exist
			// bufferedReader.close();
			// return Optional.of(price);
		} catch (IOException e) {
			System.out.println("Failure to send request.");
		} catch (InterruptedException e) {
			System.out.println("Failure with the sleep function.");
		}
		return Optional.empty();
	}
	
	public static void reset_counter() {
		counter = 0;
	}
}
