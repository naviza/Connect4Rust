import java.util.List;
import java.util.stream.Stream;
import java.util.concurrent.TimeUnit;

public class PickShareFunctional {
	public static final int MIN_PRICE = 500;
	
	public static ShareInfo findHighPriced(Stream<String> shares) {
		List<ShareInfo> shareInfo = shares.map(share -> {
											try {
												TimeUnit.SECONDS.sleep(12);
											} catch (InterruptedException e) {
												// TODO Auto-generated catch block
												e.printStackTrace();
											}
											return new ShareInfo((String) share, APIFinance.getPrice((String) share));})
										.filter(share -> share.price.intValue() > MIN_PRICE)
										.sorted((share1, share2) -> share2.price.compareTo(share1.price))
										.toList();
		return shareInfo.get(0);
	}
}