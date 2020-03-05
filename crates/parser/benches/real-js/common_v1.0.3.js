
window.YAHOO = window.YAHOO || {};
YAHOO.JP = YAHOO.JP || {};
YAHOO.JP.basic = YAHOO.JP.basic || {};
YAHOO.JP.ui = YAHOO.JP.ui || {};
YAHOO.JP.bbpromo = YAHOO.JP.bbpromo || {};

/* global */
$ = function(id){return document.getElementById(id)} || {};

if(typeof Object.create!=="function"){Object.create=function(o){var F=function(){};F.prototype=o;return new F();};}

/* basic.addEvent() */
YAHOO.JP.basic.addEvent = function(elem, event, func){
  if (elem.addEventListener) {
    elem.addEventListener(event, func, false);
  } else if (elem.attachEvent) {
    elem.attachEvent("on" + event, func);
  }
};

/* basic.browser() */
YAHOO.JP.basic.browser = function(type){
  if (type === "ie6") {
    return (typeof document.documentElement.style.maxHeight === "undefined");
  }
}

/* basic.getElementsByClassName */
YAHOO.JP.basic.getElementsByClassName = function(clsName, target){
  for(var i = 0, len = target.childNodes.length; i < len; i++) {
    var elem = target.childNodes[i];
    if ((elem.className)&&(elem.className.match("(?:^|\\s)"+ clsName +"(?:$|\\s)"))) {
      return elem;
    }
  }
};

/* ui.tab */
YAHOO.JP.ui.tab = function(){
  
  var tabObj = {
    
    id: "",
    tab: "",
    tabContents: "",
    
    init: function(tabWidth, tabHeight, tabDistance){
      
      this.tab.style.position = "absolute";
      this.tabContents.style.position = "absolute";
      this.tabContents.style.zIndex = "100";
      if (tabWidth > 0) this.tab.style.left = tabWidth + tabDistance + "px";
      this.tabContents.style.top = tabHeight + "px";
      
        if (this.tab.className.match("(?:^|\\s)current(?:$|\\s)")) {
          this.current();
        } else {
          this.ncurrent();
        }   
    },
    
    current: function(){
      
      this.tab.style.zIndex = "400";
      // this.tabContents.style.zIndex = "200";
      this.tabContents.style.display = "block";
      this.tab.className = "tab current";
      // YAHOO.JP.basic.addClass("current", this.tab);
      // YAHOO.JP.basic.removeClass("ncurrent", this.tab);
      
      this.adjustWrapperHeight();
      
    },
    
    ncurrent: function(){
      
      this.tab.style.zIndex = "300";
      // this.tabContents.style.zIndex = "100";
      this.tabContents.style.display = "none";
      this.tab.className = "tab ncurrent";
      // YAHOO.JP.basic.addClass("ncurrent", this.tab);
      // YAHOO.JP.basic.removeClass("current", this.tab);
    },
    
    change: function(_this, _tabs){
      
      return function(){
        for(var i = 0, len = _tabs.length; i < len; i++) {
          if (_tabs[i] === _this) {
            _tabs[i].current();             
          } else {
            _tabs[i].ncurrent();
          }
        }
      }
      
    },
    
    directTabShow: function() {
      
      if (this.id.id === targetTab) { 
          for(var j = 0 , len = tabs.length ; j < len; j++) {
            tabs[j].ncurrent();
          }
          
        this.current(); 
        
        if (targetPosition === 'top') {
//          window.scrollTo(0,0);
        }
        else {
          window.location.hash = targetTab;
        }
      }

    },
    
    adjustWrapperHeight: function(){
      tabsWrapper.style.height = this.tab.clientHeight + this.tabContents.clientHeight + tabsWrapperMB + "px";
    }
    
  }
  
  var tabs = [];
  var tabsWrapper = $(arguments[0]).parentNode;
  // var tabsWrapperHeight = 0;
  
  var tabDistance = 0;
  var tabsWrapperMB = 20;
  
  var targetQuery = window.location.search;
  var splitPoint = targetQuery.lastIndexOf('&');
  var targetTab;
  var targetPosition;
  
  if (splitPoint != -1) {
    targetTab = targetQuery.slice(1, splitPoint);
    targetPosition = targetQuery.slice(splitPoint + 1);
  }
  else {
    targetTab = targetQuery.slice(1); 
  }
  
  
  for(var i = 0, len = arguments.length; i < len; i++) {
    
    tabs[i] = Object.create(tabObj);
    tabs[i].id = $(arguments[i]);
    tabs[i].tab = YAHOO.JP.basic.getElementsByClassName("tab", tabs[i].id);
    tabs[i].tabContents = YAHOO.JP.basic.getElementsByClassName("tabContents", tabs[i].id);
    
    // tabsWrapperHeight = tabsWrapperHeight > tabs[i].tab.clientHeight + tabs[i].tabContents.clientHeight ? tabsWrapperHeight : tabs[i].tab.clientHeight + tabs[i].tabContents.clientHeight;
    
    tabs[i].init(i * tabs[i].tab.clientWidth, tabs[i].tab.clientHeight, tabDistance * i);

    tabs[i].directTabShow();

    YAHOO.JP.basic.addEvent(tabs[i].tab, "click", tabs[i].change(tabs[i], tabs));
            
  }
  
  // tabsWrapper.style.height = tabsWrapperHeight + tabsWrapperMB + "px";
  
}

/* ui.sScroll */
/* YAHOO.JP.ui.sScroll(elem) */
YAHOO.JP.ui.sScroll = function(){

  var elem = arguments[0];

  if (elem) {

    if (elem.tagName === "A") {
      elem.href = "javascript:void(0)";
    }
    if (elem.getElementsByTagName("a")) {
      elem.getElementsByTagName("a")[0].href = "javascript:void(0)";
    }
  
    YAHOO.JP.basic.addEvent(elem, "click", function(){
      
      var sT = document.documentElement.scrollTop || document.body.scrollTop;
      scrollEffect(sT, 0, 250);
      
      
      function scrollEffect(from, to, duration, dom, callback) {
      
        var begin = new Date() * 1;
        var amount = to - from;
    
        var effectIv = setInterval(function(){
            
          var now = new Date() * 1;
          var time = now - begin;
          var progress = amount * time / duration + from;
          
          if (progress < to) progress = to;
          // console.log(progress);
          
          window.scroll(0, progress);
          //dom.style.top = progress + "px";
            
          if (time > duration) {
            clearInterval(effectIv);
            if (callback) callback();
          }
          
        }, 1000/60);
        
      }
      
    });
  
  }

}

/* ui.sameHeight */
/* YAHOO.JP.ui.sameHeight(elem[array]) */
YAHOO.JP.ui.sameHeight = function(){
  
  var _array = arguments;
  
  if (_array.length > 0) {
    
    var minHeight = _array[0].offsetHeight;
    
    for (var i = 0, len = _array.length; i < len; i++) {
      if (_array[i+1]) {
        minHeight = Math.max(minHeight, _array[i+1].offsetHeight);
      }
    }
    for (var i = 0, len = _array.length; i < len; i++) {
      if (YAHOO.JP.basic.browser("ie6")) {
        _array[i].style.height = minHeight + "px";
      } else {
        _array[i].style.minHeight = minHeight + "px";
      }
    }
  }   
    
}

/* ui.slide */
/* YAHOO.JP.ui.slide(triggerId, targetId) */

YAHOO.JP.ui.slide = (function(trigger, target){

  var $ = function(id){return document.getElementById(id)};
  var sliding = false;
  var trigger = $(trigger);
  var target = $(target);
  var target_height = target.offsetHeight;
  
  target.className += " close";
  target.style.display = "none";
  target.style.height = "0";
  
  YAHOO.JP.basic.addEvent(trigger, "click", toggleSwitch);

  function toggleSwitch(){
    if(!target.className.match("open")){
    slide("open");
    }
    else if(!target.className.match("close")){
    slide("close");
    }
  } 

  function slide(status) {
    if (!sliding) {
      sliding = true;
      if (status === "open") {
        trigger.className = trigger.className += " openedTrigger";
        target.style.display = "block";
        slideEffect(0, target_height, 300, target, function(){
          sliding = false;
          target.className = target.className.replace("close", "open");
        });
      } else if (status === "close") {
        trigger.className = trigger.className.replace(" openedTrigger", "");
        slideEffect(target_height, 0, 300, target, function(){
          sliding = false;
          target.className = target.className.replace("open", "close");
          target.style.display = "none";
        });
      }
    }
  }

  function slideEffect(from, to, duration, dom, callback) {
    
    var begin = new Date() * 1;
    var amount = to - from;

    var slideEffect_Iv = setInterval(function(){
        
      var now = new Date() * 1;
      var time = now - begin;
      var progress = amount * time / duration + from;
      
      if (progress < 0) progress = 0;
      if (progress > target_height) progress = target_height;
      
      dom.style.height = progress + "px";
        
      if (time > duration) {
        clearInterval(slideEffect_Iv);
        if (callback) callback();
      }
      
    },1000/60);
  }
})

YAHOO.JP.bbpromo.init = (function(){
  
})();


jQuery(document).ready(function($){
  var pathname = location.pathname;
  var navLinks = $('.gnav-service--top').find('a');
  console.log(pathname);
  var navCategory;
  if(pathname.indexOf('hikari_wf') != -1) {
    navCategory = 'hikari_wf';
  }
  if(pathname.indexOf('norikae') != -1) {
    navCategory = 'norikae';
  }
  if(pathname.indexOf('adsl') != -1) {
    navCategory = 'adsl';
  }
  if(pathname.indexOf('special') != -1) {
    navCategory = 'special';
  }

  $.each(navLinks, function(index, ele) {
    var link = $(ele).attr('href');
    if(link.indexOf(navCategory) != -1) {
      $(ele).addClass('gnav-service__link--current');
    }
  });
  var pos = $('#yjContentsBody').offset().top;
  var scroll = function(e) {
    if($(window).scrollTop() > pos) {
      $('.under-nav').css('display', 'block').stop(true).animate({opacity: 1}, { duration: 100, queue: false});
    } else {
      $('.under-nav').stop(true).animate({opacity: 0},{ duration: 10, queue: false, easing: 'linear', complete: function() {
        $(this).css('display', 'none');
      }});
    }
  }
  $(window).on('scroll', scroll);
});