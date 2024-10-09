type HastElement = {
	type: "element";
	tagName: string;
	properties?: Record<string, string>;
	children: (HastElement | HastText)[];
};

type HastText = {
	type: "text";
	value: string;
};
