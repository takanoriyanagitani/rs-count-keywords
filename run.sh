#!/bin/sh

inputdata=/usr/share/dict/words

export ENV_MATCH_KIND=standard
export ENV_MATCH_KIND=std

export ENV_MATCH_KIND=first
export ENV_MATCH_KIND=left_most_first

export ENV_MATCH_KIND=long
export ENV_MATCH_KIND=left_most_longest

cat "${inputdata}" |
	./rs-count-keywords \
		apple \
		banana \
		chocolate \
		drink \
		egg \
		fruit \
		gum \
		honey \
		ice \
		juice \
		kebab \
		lemon \
		mango \
		noodle \
		orange \
		pineapple \
		quinoa \
		raspberry \
		salad \
		tofu \
		udon \
		vanilla \
		whisky \
		xylitol \
		yellowberry \
		zucchini |
	jq -c
