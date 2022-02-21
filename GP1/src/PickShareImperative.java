import java.util.function.Predicate;

public class PickShareImperative {
	public static void findHighPriced() {
		ShareInfo highPriced = null;
		final Predicate<ShareInfo> isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
				for (String symbol : Shares.symbols) {
					ShareInfo shareInfo = ShareUtil.getPrice(symbol);
					if (isPriceLessThan500.test(shareInfo))
						highPriced = ShareUtil.pickHigh(highPriced, shareInfo);
				}
		System.out.println("High priced under $500 is" + highPriced);
	}
}
