import java.math.BigDecimal;
import java.util.Optional;
import java.util.concurrent.TimeUnit;
import java.util.stream.Stream;

// https://belief-driven-design.com/functional-programming-with-java-exception-handling-e69997c11d3/

public class PickShareFunctional {
	private static final BigDecimal MIN_PRICE = new BigDecimal(500);
	
	public static Optional<ShareInfo> findHighPriced(Stream<String> shares) {
		Optional<ShareInfo> shareInfo = shares.map(share ->  new ShareInfo((String) share, APIFinance.getPrice((String) share)))
										.filter(share -> share.price.isPresent())
										.filter(share -> share.price.get().compareTo(MIN_PRICE) > 0)
										.reduce((share1, share2) -> ShareUtil.pickHigh(share1, share2));
		return shareInfo;
	}
	
	public static final BigDecimal getMinPrice() {
		return MIN_PRICE;
	}
}