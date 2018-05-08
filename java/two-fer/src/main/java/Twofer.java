class Twofer {
    String twofer(String name) {
	    if(null == name || name.isEmpty()) {
		    name = "you";
	    }

	    return "One for " + name + ", one for me.";
    }
}
