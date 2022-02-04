import java.util.List;
import java.util.stream.Stream;
import java.util.stream.Collectors;

// https://belief-driven-design.com/functional-programming-with-java-exception-handling-e69997c11d3/

public class PickShareFunctional {
	public static final int MIN_PRICE = 500;
	
	public static ShareInfo findHighPriced(Stream<String> shares) {
		List<ShareInfo> shareInfo = shares.map(share -> new ShareInfo((String) share, APIFinance.getPrice((String) share)))
										.filter(share -> share.price.isPresent())
										.filter(share -> share.price.get().intValue() > MIN_PRICE)
										.sorted((share1, share2) -> share2.price.get().compareTo(share1.price.get()))
										.collect(Collectors.toList());
										// .toList();
		return shareInfo.get(0);
	}
}