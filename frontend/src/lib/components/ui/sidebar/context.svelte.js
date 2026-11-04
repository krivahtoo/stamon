import { IsMobile } from "$lib/hooks/is-mobile.svelte.js";
import { getContext, setContext } from "svelte";
import { SIDEBAR_KEYBOARD_SHORTCUT } from "./constants.js";

class SidebarState {
	/** @type {{ open: () => boolean, setOpen: (value: boolean) => void }} */
	props;
	open = $derived.by(() => this.props.open());
	openMobile = $state(false);
	/** @type {(value: boolean) => void} */
	setOpen;
	#isMobile;
	state = $derived.by(() => (this.open ? "expanded" : "collapsed"));

	/** @param {{ open: () => boolean, setOpen: (value: boolean) => void }} props */
	constructor(props) {
		this.setOpen = props.setOpen;
		this.#isMobile = new IsMobile();
		this.props = props;
	}

	// Convenience getter for checking if the sidebar is mobile
	// without this, we would need to use `sidebar.isMobile.current` everywhere
	get isMobile() {
		return this.#isMobile.current;
	}

	// Event handler to apply to the `<svelte:window>`
	/** @param {KeyboardEvent} e */
	handleShortcutKeydown = (e) => {
		if (e.key === SIDEBAR_KEYBOARD_SHORTCUT && (e.metaKey || e.ctrlKey)) {
			e.preventDefault();
			this.toggle();
		}
	};

	/** @param {boolean} value */
	setOpenMobile = (value) => {
		this.openMobile = value;
	};

	toggle = () => {
		return this.#isMobile.current
			? (this.openMobile = !this.openMobile)
			: this.setOpen(!this.open);
	};
}

const SYMBOL_KEY = "scn-sidebar";

/**
 * Instantiates a new `SidebarState` instance and sets it in the context.
 *
 * @param {{ open: () => boolean, setOpen: (value: boolean) => void }} props The constructor props for the `SidebarState` class.
 * @returns  The `SidebarState` instance.
 */
export function setSidebar(props) {
	return setContext(Symbol.for(SYMBOL_KEY), new SidebarState(props));
}

/**
 * Retrieves the `SidebarState` instance from the context. This is a class instance,
 * so you cannot destructure it.
 * @returns {SidebarState} The `SidebarState` instance.
 */
export function useSidebar() {
	return /** @type {SidebarState} */ (getContext(Symbol.for(SYMBOL_KEY)));
}