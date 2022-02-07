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

	public static synchronized Optional<BigDecimal> getPrice(final String symbol) {
		try {
			// Sleep to limit API calls to 5 per minute
			TimeUnit.SECONDS.sleep(12);
			// define the URL for API Access
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
}
