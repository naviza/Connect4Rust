import java.math.BigDecimal;
import java.util.Optional;
import java.util.stream.Stream;

// https://belief-driven-design.com/functional-programming-with-java-exception-handling-e69997c11d3/

public class PickShareFunctional {
	// Minimum price to consider for the highest price share
	private static final BigDecimal MIN_PRICE = new BigDecimal(500);
	
	// Return either the highest price share above a minimum price or nothing if that criteria can't be met
	public static Optional<ShareInfo> findHighPriced(Stream<String> shares) {
		// Map each share to it's corresponding price
		Optional<ShareInfo> shareInfo = shares.map(share ->  new ShareInfo((String) share, APIFinance.getPrice((String) share)))
										.filter(share -> share.price.isPresent())  // Ensure that each share has a price
										.filter(share -> share.price.get().compareTo(MIN_PRICE) > 0)  // Filter out the shares below the minimum price
										.reduce((share1, share2) -> ShareUtil.pickHigh(share1, share2));  // From the remaining shares pick the one with the highest price
		return shareInfo;  // Return either that share with the highest price above a set minimum or nothing if that criteria is not meet
	}
	
	// Return the minimum price considered for the highest price share
	public static final BigDecimal getMinPrice() {
		return MIN_PRICE;
	}
}