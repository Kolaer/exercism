import java.util.HashMap;

class ArmstrongNumbers {

    boolean isArmstrongNumber(int numberToCheck) {
        int num = numberToCheck;

        HashMap<Integer, Integer> map = new HashMap<>();

        while (num != 0) {
            int mod = num % 10;
            num /= 10;

            if (!map.containsKey(mod)) {
                map.put(mod, 0);
            }

            int count = map.get(mod);

            map.put(mod, count + 1);
        }

        int len = map.values().stream()
                .reduce((x, y) -> x + y)
                .orElse(0);

        Double sum = map.entrySet().stream()
                .mapToDouble(x -> {
                    Integer count = x.getValue();
                    Integer value = x.getKey();

                    return count * Math.pow(value, len);
                }).sum();

        return sum.intValue() == numberToCheck;
    }

}
