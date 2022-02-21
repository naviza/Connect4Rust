import java.math.BigDecimal;
import java.util.Optional;

public class ShareInfo {
	public final String symbol;
	public final Optional<BigDecimal> price;
	
	public ShareInfo(final String theSymbol, final Optional<BigDecimal> thePrice) {
		symbol = theSymbol;
		price = thePrice;
	}
	
	public String toString() {
		return String.format("symbol: %s price: %g", symbol, price.get());
	}
}
