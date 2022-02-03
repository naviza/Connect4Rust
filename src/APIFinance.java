import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.math.BigDecimal;
import java.net.URL;
import java.net.URLConnection;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class APIFinance {
	private static final String BASE_URL = "https://www.alphavantage.co/query?";
	private final static String apiKey = "0BEBA6Z10REBY4Z5";
	
	public static BigDecimal getPrice(final String symbol) {
		BigDecimal price = new BigDecimal(0);
		try {
			URL url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
			URLConnection connection = url.openConnection();
			InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
			BufferedReader bufferedReader = new BufferedReader(inputStream);
			String line;
			while ((line = bufferedReader.readLine()) != null) {
				if (line.contains("price")) {
					price = new BigDecimal(line.split("\"")[3].trim());
				}
			}
			bufferedReader.close();
		} catch (IOException e) {
			System.out.println("failure sending requests");
		}
		return price;
	}


	public static BigDecimal getPrice2(final String symbol) {
		BigDecimal price = new BigDecimal(0);
		try {
			// URL url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
			BufferedReader bufferedReader = new BufferedReader(new InputStreamReader((new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey)).openConnection().getInputStream(), "UTF-8"));
			price = new BigDecimal((bufferedReader.lines().filter(line -> line.contains("price")).findFirst().map(value -> value.split("\"")[3].trim()).orElse("-1")));
			bufferedReader.close();
		} catch (IOException e) {
			System.out.println("failure sending requests");
		}
		return price;
	}

	public static Optional<BigDecimal> getPriceOptional(final String symbol) {
		BufferedReader bufferedReader;
		try {
			bufferedReader = new BufferedReader(new InputStreamReader((new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey)).openConnection().getInputStream(), "UTF-8"));
			BigDecimal price = new BigDecimal((bufferedReader.lines().filter(line -> line.contains("price")).findFirst().map(value -> value.split("\"")[3].trim()).orElse("-1")));
			bufferedReader.close();
			return Optional.of(price);
		} catch (IOException e) {
			// e.printStackTrace();
		}
		return Optional.empty();
	}
}
