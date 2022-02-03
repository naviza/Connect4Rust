import java.util.List;
import java.util.stream.Stream;
import java.util.stream.Collectors;
import java.util.concurrent.TimeUnit;

// https://belief-driven-design.com/functional-programming-with-java-exception-handling-e69997c11d3/

public class PickShareFunctional {
	public static final int MIN_PRICE = 500;
	
	// public static ShareInfo findHighPriced(Stream<String> shares) {
	// 	List<ShareInfo> shareInfo = shares.map(share -> {
	// 										try {
	// 											TimeUnit.SECONDS.sleep(2);
	// 										} catch (InterruptedException e) {
	// 											e.printStackTrace();
	// 										}
	// 										return new ShareInfo((String) share, APIFinance.getPrice2((String) share));})
	// 									.filter(share -> share.price.intValue() > MIN_PRICE)
	// 									.sorted((share1, share2) -> share2.price.compareTo(share1.price))
	// 									.collect(Collectors.toList());
	// 									// .toList();
	// 	return shareInfo.get(0);
	// }

	public static ShareInfo findHighPriced2(Stream<String> shares) {
		List<ShareInfo> shareInfo = shares.map(share -> new ShareInfo((String) share, APIFinance.getPriceOptional((String) share)))
										.filter(share -> share.price.isPresent())
										.filter(share -> share.price.get().intValue() > MIN_PRICE)
										.sorted((share1, share2) -> share2.price.get().compareTo(share1.price))
										.collect(Collectors.toList());
										// .toList();
		return shareInfo.get(0);
	}
}