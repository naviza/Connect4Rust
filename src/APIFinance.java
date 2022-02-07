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
			TimeUnit.SECONDS.sleep(12);
			URL url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
			URLConnection connection = url.openConnection();
			InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
			BufferedReader bufferedReader = new BufferedReader(inputStream);
			BigDecimal price = new BigDecimal((bufferedReader.lines().filter(line -> line.contains("price")).findFirst().map(value -> value.split("\"")[3].trim()).orElse("0")));
			bufferedReader.close();
			return Optional.of(price);
		} catch (IOException e) {
			System.out.println("failure sending requests");
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
		return Optional.empty();
	}
}
