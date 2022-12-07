import {BaseElement, html, css, ScrollbarStyle} from '[FLOW-UX-PATH]';

let isTouchCapable = 'ontouchstart' in window ||
	window.DocumentTouch && document instanceof window.DocumentTouch;/* ||
	navigator.maxTouchPoints > 0 ||
	window.navigator.msMaxTouchPoints > 0;*/
//isTouchCapable = true;

ScrollbarStyle.appendTo("head");

let mobileLayoutMaxWidth = 1200;

export class WorkflowAppLayout extends BaseElement{
	static get properties(){
		return {
			"menu-icon":{type:String},
			"right-drawer-toggle-icon":{type:String},
			"min-left-drawer":{type:Boolean, _reflect:true},
			"swipe-threshold": {type: Number},
			"scroll-margin": {type: Number},
			"mobile-layout-max-width":{type: Number},
			"hide-right-drawer":{type:Boolean},
			"hide-bottom-menu":{type:Boolean},
			"hide-menu-toggler":{type:Boolean},
			"basic-layout":{type:Boolean}
		}
	}
	static get styles(){
		return [ScrollbarStyle, css`
		:host{
			display:flex;
			flex-direction:column;
			align-items:stretch;
			width:var(--flow-app-width, 100vw);
			height:var(--flow-app-height, 100vh);
			box-sizing:border-box;
			--drawer-gap:60px;
		}
		.drawer{
			--flow-scrollbar-width:2px;
		}

		.header-outer{
			display:var(--flow-app-header-outer-display, flex);
			flex-direction:var(--flow-app-header-outer-flex-direction, row);
			align-items:var(--flow-app-header-outer-align-items, center);
			overflow-y:var(--flow-app-header-outer-overflow-y, auto);
			overflow-x:var(--flow-app-header-outer-overflow-x, auto);
			padding:var(--flow-app-header-outer-padding, 5px 10px);
			border-bottom:var(--flow-app-header-border-bottom, 1px solid var(--flow-border-color));
		}
		.header{
			display:flex;
			flex:var(--flow-app-header-flex, 1);
			flex-direction:var(--flow-app-header-flex-direction, row);
			align-items:var(--flow-app-header-align-items, center);
			overflow-y:var(--flow-app-header-overflow-y, auto);
			overflow-x:var(--flow-app-header-overflow-x, auto);
			max-height:var(--flow-app-header-max-height, 10vh);
			min-height:var(--flow-app-header-min-height, 55px);
		}
		.menu-btn{
			margin:var(--flow-app-menu-btn-margin, 0px 15px);
			cursor:pointer;
		}
		.outer{
			flex:1;
			display:flex;
			overflow:var(--flow-app-outer-overflow, hidden);
			flex-direction:row;
			align-items:stretch;
			position:var(--flow-app-outer-position, relative);
		}
		.main{
			flex:1;
			position:relative;
			overflow-y:var(--flow-app-main-overflow-y, auto);
			overflow-x:var(--flow-app-main-overflow-x, auto);
			padding:var(--flow-app-main-padding, 15px);
			border-top:var(--flow-app-main-border-top, 0px);
			max-width: var(--flow-app-main-max-width, 900px);
    		margin: var(--flow-app-main-margin, 0px auto);
		}
		header ::slotted(flow-caption-bar){
			--flow-caption-bar-width:auto;
			--flow-caption-bar-host-width:auto;
		}
		.left-drawer{
			box-sizing: border-box;
			width:var(--flow-app-left-drawer-width, 300px);
			overflow-y:var(--flow-app-left-drawer-overflow-y, auto);
			overflow-x:var(--flow-app-left-drawer-overflow-x, auto);
			border-right:var(--flow-app-left-drawer-border-right, 1px solid var(--flow-border-color));
			border-top:var(--flow-app-left-drawer-border-top, 0px);
		}
		:host(:not(.no-transition)) .left-drawer{
			transition:var(--flow-app-left-drawer-transition, width 0.2s ease);
		}
		.right-drawer{
			width:var(--flow-app-right-drawer-width, 300px);
			overflow-y:var(--flow-app-right-drawer-overflow-y, auto);
			overflow-x:var(--flow-app-right-drawer-overflow-x, auto);
			border-left:var(--flow-app-right-drawer-border-left, 1px solid var(--flow-border-color));
			background-color:var(--flow-app-active-drawer-bg, var(--flow-background-color));
			box-sizing: border-box;
		}
		:host(:not(.no-transition)) .right-drawer{
			transition:var(--flow-app-right-drawer-transition, margin-right 0.2s ease);
		}
		:host(.left-drawer-floating) .main{
			margin-left:var(--flow-app-left-drawer-min-size, 76px);
		}
		:host(.left-drawer-floating) .left-drawer{
			position:absolute;top:0px;bottom:0px;
			z-index:var(--flow-app-left-drawer-z-index, 9010);
			width:var(--flow-app-left-drawer-min-size, 70px);
			background-color:var(--flow-app-drawer-bg, var(--flow-background-color));
		}
		:host(.left-drawer-floating) .left-drawer:hover{
			width:var(--flow-app-left-drawer-width, 300px);
			background-color:var(--flow-app-active-drawer-bg, var(--flow-background-color));
		}

		:host(.right-drawer-floating) .right-drawer{
			margin-right:calc(-1 * var(--flow-app-right-drawer-width, 300px));
		}
		:host(.right-drawer-floating.right-open) .right-drawer{
			margin-right:0px;
		}

		.right-drawer-toggler{
			position:fixed;
			bottom:var(--flow-app-right-drawer-toggler-bottom, 20px);
			right:var(--flow-app-right-drawer-toggler-right, 20px);
			border-radius:50%;
			width:40px;height:40px;
			background:var(--flow-app-right-drawer-toggler-bg, var(--flow-background-color, #FFF));
			box-shadow:var(--flow-box-shadow);
			--flow-iconbtn-padding:2px;
			--flow-btn-wrapper-min-width:100%;
			z-index:9002;
		}

		.bottom-menu{
			display:none;
			width:100%;
			height:75px;
			position: absolute;bottom:0px;left:0px;right:0px;
			z-index:var(--flow-app-tabs-z-index, 9002);
		}
		.mask{
			display:none;
			position:absolute;left:0px;bottom:0px;right:0px;top:0px;
			background-color:var(--flow-app-mask-bg, rgba(255, 255, 255, 0.5));
			z-index:calc(var(--flow-app-tabs-z-index, 9002) + 1);
		}

		@media screen and (max-width:400px){
			:host{
				--drawer-gap:50px;
			}
		}

		:host(.small-screen){
			--drawer-width:var(--flow-app-left-drawer-width, calc(100% - var(--drawer-gap)));
		}
		:host(.small-screen) .main{
			margin-right:0px;
			margin-left:0px;
			margin-bottom:var(--flow-app-tabs-height, 60px);
			min-width:100%;
		}
		:host(.small-screen) .bottom-menu{
			display:block;
		}
		:host(.small-screen) .left-drawer,
		:host(.small-screen) .right-drawer{
			position:relative;
			width:var(--drawer-width);
			min-width:var(--drawer-width);
			z-index:var(--flow-app-right-drawer-z-index, 9010);
		}
		:host(.small-screen) .left-drawer{
			margin-left:calc(var(--drawer-width) * -1);
			z-index:var(--flow-app-left-drawer-z-index, 9010);
			background-color:var(--flow-app-active-drawer-bg, var(--flow-background-color));
			border:var(--flow-app-left-drawer-border, 0px);
			box-shadow:var(--flow-app-left-drawer-box-shadow, var(--flow-box-shadow));
		}

		:host(.small-screen) .right-drawer{
			border:0px;
			box-shadow:var(--flow-app-right-drawer-box-shadow, var(--flow-box-shadow));
		}

		/******* left-drawer *******/
		:host(.small-screen:not(.no-transition)) .left-drawer{
			transition:var(--flow-app-left-drawer-transition, all 0.2s ease);
		}
		:host(.small-screen.left-drawer-open) .left-drawer{
			margin-left:0px;
		}

		/******* right-drawer *******/
		:host(.small-screen.right-drawer-open) .left-drawer{
			margin-left:calc(var(--drawer-width) * -2)
		}

		:host(.small-screen.left-drawer-open) .mask,
		:host(.small-screen.right-drawer-open) .mask{
			display:block;
		}

		@media screen and (min-width:768px){
			:host,
			:host(.small-screen){
				--drawer-width:350px;
			}
		}
		`]
	}
	render(){
		if (this["basic-layout"]){
			return html`
				<header class="header-outer">
					<slot name="header-prefix"></slot>
					<slot name="header-prefix2"></slot>
					<div class="header"><slot name="header"></slot></div>
					<slot name="header-suffix"></slot>
				</header>
				<div class="outer">
					<div class="drawer left-drawer">
						<slot name="left-drawer"></slot>
					</div>
					<div class="main"><slot id="slot-main" name="main"></slot></div>
					<div class="mask" @click=${this.onMaskClick}></div>
				</div>
			`;
		}else{
			return html`
				<header class="header-outer">
					<slot name="header-prefix"></slot>
					${
						this["hide-menu-toggler"]? '':
						html`<fa-icon class="menu-btn"
							icon="${this['menu-icon'] || 'bars'}" 
							@click="${this.toggleFloatingLeftDrawer}"></fa-icon>`
					}
					<slot name="header-prefix2"></slot>
					<div class="header"><slot name="header"></slot></div>
					<slot name="header-suffix"></slot>
				</header>
				<div class="outer">
					<div class="drawer left-drawer">
						<slot name="left-drawer"></slot>
					</div>
					<div class="main"><slot id="slot-main" name="main"></slot></div>
					${
						this["hide-right-drawer"]? '':
						html`<div class="drawer right-drawer"><slot name="right-drawer"></slot></div>
						<flow-btn class="right-drawer-toggler"
							@click=${this.toggleFloatingRightDrawer}
							icon="${this['right-drawer-toggle-icon']}">
						</flow-btn>`
					}
					${
						this["hide-bottom-menu"]? '':
						html`<div class="bottom-menu"><slot name="bottom-nav"></slot></div>`
					}
					<div class="mask" @click=${this.onMaskClick}></div>
				</div>
			`;
		}
	}

	constructor(){
		super();
		
		if (window.location.href.includes("app-log")){
			this.logEl = document.createElement("pre");
			this.logEl.setAttribute("class", "app-log");
			document.body.appendChild(this.logEl);
		}
		this.nonSwipeableSources = '.not-swipeable';//'flow-dropdown,.menu-item,flow-select,flow-selector,flow-input,flow-checkbox,select,textarea, input,.not-swipeable';
	}

	cleanAppLog(...args){
		if (this.logEl){
			this.logEl.innerHTML = "";
		}
	}
	appLog(...args){
		if (this.logEl){
			this.logEl.innerHTML += args.join("\n")+"\n";
		}
	}

	firstUpdated(...args){
		this.isReady = true;
		this.fire("ready");
		this.swipeThreshold = this["swipe-threshold"] || 100;
		super.firstUpdated(...args);
		this.mobileLayoutQueryEl = this.renderRoot.querySelector(".mobile-layout-query");
		this.outerEl = this.renderRoot.querySelector(".outer");
		this.leftDrawer = this.renderRoot.querySelector(".left-drawer");
		this.mainEl = this.renderRoot.querySelector(".main");
		let slotMain = this.renderRoot.querySelector("#slot-main");
		slotMain.addEventListener("slotchange", ()=>{
			//let scrollable = this.mainEl.querySelector(".scrollable");
			let scrollable = slotMain
				.assignedElements()
				.filter(item=>item.matches(".scrollable"))[0]
			//alert("scrollable"+scrollable)
			if (scrollable){
				//window.xxxx = scrollable;
				scrollable.addEventListener("scroll", (e)=>{
					this.onMainScroll(e, scrollable);
				})
				if (window.MutationObserver){
					const observer = new MutationObserver((e)=>{
						//scrollable.scrollTo({left:0, top:0, behavior:"smooth"})
						this.onMainScroll(e, scrollable);
						/*
						setTimeout(()=>{
							scrollable.scrollTo({left:0, top:0, behavior:"smooth"})
						}, 100)
						*/
					});
					// Start observing the target node for configured mutations
					observer.observe(scrollable, {childList: true});
				}
			}
		})
		
		
		this.tabContentsEl = this.renderRoot.querySelector(".tab-contents");
		this.tabContentSlot = this.renderRoot.querySelector('slot[name="tab-content"]')
		this.onResize();
		this.initSwipeable();
	}

	connectedCallback(){
		super.connectedCallback();
		this.toggleFloatingRightDrawer();
		this.resizeObserver = new ResizeObserver((entries) => {
			this.onResize();
		});

		this.resizeObserver.observe(this);
	}

	onMaskClick(e){
		if (!this.wasDragged){
			this.closeLeftDrawer();
			this.closeRightDrawer();
		}
		this.wasDragged = false
	}

	onMainScroll(e, el){
		this._onMainScroll(e, el);
		setTimeout(()=>{
			this._onMainScroll(e, el);
		}, 1)
	}
	_onMainScroll(e, el){
		let {scrollHeight, scrollTop, clientHeight} = el;
		let s = scrollHeight - Math.ceil(scrollTop);
		//this.cleanAppLog();
		//this.appLog(`onMainScroll:${s}, scrollTop:${scrollTop}, scrollHeight:${scrollHeight}, t:${Date.now()}`)
		//let scrolled = s <= clientHeight+100;
		//console.log("s==clientHeight", s, clientHeight)
		this.classList.toggle("almost-scrolled", s <= clientHeight+(this["scroll-margin"]||50));
		this.classList.toggle("full-scrolled", s <= clientHeight+2);
		//console.log("e.scroll:"+s+", scrolled:"+scrolled+", clientHeight:"+clientHeight);
	}

	onResize(){
		//this.classList.toggle("menu-over", this.isSmallScreen());
		if (this.isSmallScreen()){
			this.classList.add("small-screen");
			this.classList.remove("left-drawer-floating", "right-drawer-floating");
			this.closeRightDrawer();
			this.closeLeftDrawer();
		}else{
			this.classList.remove("small-screen");
			this.classList.add("right-drawer-floating");
		}
	}

	isSmallScreen(){
		//console.log("xxxxx", this.mobileLayoutQueryEl.getBoundingClientRect().width)
		//return this.mobileLayoutQueryEl.getBoundingClientRect().width > 0;
		let maxWidth = this["mobile-layout-max-width"] || mobileLayoutMaxWidth;
		return window.matchMedia(`(max-width:${maxWidth}px)`).matches;
		//return window.innerWidth <= mobileLayoutMaxWidth;
		//return this.getBoundingClientRect().width < mobileLayoutMaxWidth;
	}

	onMenuMouseEnter(){
		if (!this.isSmallScreen()){
			//this.classList.add("menu-over")
		}
	}
	onMenuMouseLeave(){
		if (!this.isSmallScreen()){
			//this.classList.remove("menu-over")
		}
	}
	_showRightDrawer(show=true){
		this.rightDrawerOpen = show;
		if (this.rightDrawerOpen){
			this.classList.add("right-drawer-open");
		}else{
			this.classList.remove("right-drawer-open");
		}
		//if (!this.rightDrawerOpen){
			this.leftDrawer.style.removeProperty("margin-left");
		//}
	}
	toggleFloatingRightDrawer(){
		this.classList.add("right-drawer-floating");
		this.rightDrawerOpen = !this.rightDrawerOpen;
		this.classList.toggle("right-open", this.rightDrawerOpen);
	}
	toggleRightDrawer(){
		this._showRightDrawer(!this.rightDrawerOpen)
	}
	closeRightDrawer(){
		this._showRightDrawer(false)
	}
	openRightDrawer(){
		this._showRightDrawer(true)
	}
	_showLeftDrawer(show=true){
		this.drawerOpen = show;
		if (this.drawerOpen){
			this.classList.add("left-drawer-open");
		}else{
			this.classList.remove("left-drawer-open");
		}
		
		//if (!this.drawerOpen){
			this.leftDrawer.style.removeProperty("margin-left");
		//}
	}
	toggleFloatingLeftDrawer(){
		this.classList.toggle("left-drawer-floating");
	}
	toggleLeftDrawer(){
		this._showLeftDrawer(!this.drawerOpen)
	}
	closeLeftDrawer(){
		this._showLeftDrawer(false)
	}
	openLeftDrawer(){
		this._showLeftDrawer(true)
	}

	get tabContentElements(){
		return this.tabContentSlot
			.assignedElements()
			.filter(item=>item.matches('[data-tab]'))
	}

	openTabContent(tabName){
		let contentElements = this.tabContentElements;
		if(!contentElements.length)
			return false;
		contentElements.forEach(el=>{
			if(el.dataset.tab == tabName){
				el.classList.add("active")
			}else{
				el.classList.remove("active")
			}
		})
	}

	closeTabContent(tabName){
		let contentElements = this.tabContentElements;
		if(!contentElements.length)
			return false;
		contentElements.forEach(el=>{
			if(el.dataset.tab == tabName){
				el.classList.remove("active")
			}
		})
	}

	initSwipeable(){
		let elms = [this.outerEl];//[this.tabContentsEl, this.leftDrawer, this.mainEl];
		//el.style.setProperty("--swipeable-n", this.count);
		//this.updateFixedPositionsOffset();

		//let onResize = this.onResize.bind(this);
		let onTouchStart = this.onTouchStart.bind(this);
		let onDrag = this.onDrag.bind(this);
		let onTouchEnd = this.onTouchEnd.bind(this);

		//el.addEventListener("resize", onResize, false);

		elms.forEach(el=>{
			let events = ["mousedown", "mousemove", "mouseup", "mouseout"];
			let isMouseEvent = true;
			if (isTouchCapable){
				isMouseEvent = false;
				events = ["touchstart", "touchmove", "touchend", "touchcancel"];
			} 
			el.addEventListener(events[0], (e)=>{
				onTouchStart(e, el)
			}, false);
			el.addEventListener(events[1], (e)=>{
				onDrag(e, el, isMouseEvent)
			}, false);
			el.addEventListener(events[2], (e)=>{
				onTouchEnd(e, el)
			}, false);
			el.addEventListener(events[3], (e)=>{
				onTouchEnd(e, el)
			}, false);
		})
	}

	unifyEvent(e) {
		return e.changedTouches ? e.changedTouches[0] : e;
	}

	isValidSwipeEvent(e){
		return !e.target.closest(this.nonSwipeableSources)
	}

	onTouchStart(e) {
		let smallScreen = this.isSmallScreen()//getComputedStyle(this).getPropertyValue("--mobile-layout");
		if(!smallScreen || !this.isValidSwipeEvent(e))
			return
		
		this.cleanAppLog();
		//alert("window width:"+window.innerWidth)
		//console.log("onTouchStart: window.innerWidth", window.innerWidth, mobileLayoutMaxWidth)
		let ev = this.unifyEvent(e);
		let startX = ev.clientX;
		let startY = ev.clientY;
		this.swipeableStarted = {
			startX,
			startY,
			mouseDown:true,
			drawerWidth: this.leftDrawer.getBoundingClientRect().width
		};
	}

	onDrag(e, el, isMouseEvent=false) {
		if (!this.swipeableStarted)
			return
		let {
			drawerWidth, mouseDown,
			dragged, _dragged, startX, startY
		} = this.swipeableStarted;

		let ev = this.unifyEvent(e);
		let x = ev.clientX;
		let y = ev.clientY;
		let moveXRound = Math.round(x - startX);
		let moveYRound = Math.round(y - startY);
		let moveY =  Math.abs(moveYRound);
		let sign = Math.sign(moveXRound);
		let moveX = Math.min(Math.abs(moveXRound), drawerWidth);
		//alert("verticalMove:"+verticalMove+", move:"+abs)
		
		//this.appLog(
		//	`onDrag:move: (${moveX}, ${moveY}), drawerWidth: ${drawerWidth}`
		//)

		if(!dragged){
			if ( moveX < 5 ||  moveY > 10 || (!this.drawerOpen && this.hideRightDrawer && moveXRound<0)){
				if (!_dragged){
					this.appLog(`drag failed: (${moveX}, ${moveY})`);
				}
				if (!isMouseEvent || !mouseDown){
					this.classList.remove("no-transition");
					this.swipeableStarted = false;
				}
				return
			}else{
				this.appLog(`drag success:(${moveX}, ${moveY})`);
				this.swipeableStarted.dragged = true;
			}
			this.swipeableStarted._dragged = true;
		}
		e.preventDefault();
		//return;
		this.classList.add("no-transition");
		let m = moveX * sign;
		let marginLeft = -1 * drawerWidth+m;
		if (this.drawerOpen){
			marginLeft = Math.min(m, 0);
		}else if (this.rightDrawerOpen){
			marginLeft = -1 * Math.min(drawerWidth*2 - m, drawerWidth*2);
		}

		//this.appLog("marginLeft:"+marginLeft)
		
		this.leftDrawer.style.marginLeft = `${marginLeft}px`;
		
	}

	onTouchEnd(e, el) {
		if (!this.swipeableStarted)
			return
		
		this.swipeableStarted.mouseDown = false;
		if (!this.swipeableStarted.dragged){
			return
		}
		this.wasDragged = true;
		e.preventDefault();
		this.classList.remove("no-transition");

		let {drawerWidth, startX} = this.swipeableStarted;
		let dx = this.unifyEvent(e).clientX - startX;
		let dxAbs = Math.abs(dx);
		
		let valid = dxAbs > this.swipeThreshold;
		this.appLog("dxAbs:"+dxAbs+", valid:"+valid)
		let isRightMove = Math.sign(dx) > 0;

		let finalize = (panel, open)=>{
			let action = `${panel}-${open?"open":"close"}`;
			this.appLog("action:"+action)
			switch(action){
				case "left-open":
					this.leftDrawer.style.setProperty("margin-left", "0px");
					this.openLeftDrawer();
				break;
				case "left-close":
					this.leftDrawer.style.setProperty("margin-left", `-${drawerWidth}px`);
					this.closeLeftDrawer();
				break;
				case "right-open":
					this.leftDrawer.style.setProperty("margin-left", `-${drawerWidth*2}px`);
					this.openRightDrawer();
				break;
				case "right-close":
					this.leftDrawer.style.setProperty("margin-left", `-${drawerWidth}px`);
					this.closeRightDrawer();
				break;
			}
		}

		if(isRightMove){
			if (this.rightDrawerOpen){//close right panel
				finalize("right", !valid);
			}else{//open left panel
				finalize("left", valid);
			}
		}else{//on left move
			if (this.drawerOpen){//close left panel
				finalize("left", !valid);
			}else{//open right panel
				finalize("right", valid);
			}
		}

		this.swipeableStarted = false;
	}

}

WorkflowAppLayout.define('workflow-app-layout');
window.WorkflowAppLayout = WorkflowAppLayout;
