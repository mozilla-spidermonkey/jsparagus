/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "/wps/build/webpack/";
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = 16);
/******/ })
/************************************************************************/
/******/ ([
/* 0 */,
/* 1 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(3);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 2 */,
/* 3 */
/***/ (function(module, exports) {

var g;

// This works in non-strict mode
g = (function() {
	return this;
})();

try {
	// This works if eval is allowed (see CSP)
	g = g || Function("return this")() || (1, eval)("this");
} catch (e) {
	// This works if the window reference is available
	if (typeof window === "object") g = window;
}

// g can still be undefined, but nothing to do about it...
// We return undefined, instead of nothing here, so it's
// easier to handle this case. if(!global) { ...}

module.exports = g;


/***/ }),
/* 4 */
/***/ (function(module, exports) {

/* global $ document */

/**
 * Masthead and SideNav Interactions
 * @type {{init}}
 */
var masthead = (function () {
  // ---------------------
  // Search Box Utilities
  // ---------------------

  /**
   * Interaction response to open the search box from the icon.
   */
  function searchExpand() {
    var searchInput = document.querySelector('.masthead input[type=text]');
    searchInput.focus();
    $(searchInput).addClass('open');
  }

  /**
   * Close the search at masthead and show the icon again.
   */
  function searchCollapse() {
    var searchInput = document.querySelector('.masthead input[type=text]');
    $(searchInput).removeClass('open');
  }

  /**
   * Add handler to masthead's search button.
   * Focus out handler.
   */
  function setupSearch() {
    var search = document.querySelector('form.searchbox');
    $(search).on('click', searchExpand);
    $(search).on('focusout', function (evt) {
      var searchBox = this;
      var isContained = searchBox.contains(evt.relatedTarget);

      if (!isContained) {
        searchCollapse();
      }

      return isContained;
    });
  }

  /**
   * Show user account popup (mobile login and logged-in state).
   * @param {MouseEvent} e
   */
  function showPopup(e) {
    e.preventDefault();
    var popup = document.querySelectorAll('.account-popup');
    var jqPopup = $(popup);
    if (!jqPopup.hasClass('show')) {
      jqPopup.addClass('show');
      $(this).attr('aria-expanded', 'true');
    }
  }

  /**
   * Close user account popup.
   */
  function hidePopup() {
    var popup = document.querySelectorAll('.account-popup');
    var jqPopup = $(popup);
    if (jqPopup.hasClass('show')) {
      jqPopup.removeClass('show');
      $('.account-links').removeAttr('aria-expanded');
    }
  }

  /**
   * Connect event handlers for user account popup behavior.
   */
  function setupLogin() {
    var userIcon = document.querySelectorAll('.paywall-links .main.account-links');
    var closeIcon = document.querySelectorAll('.account-popup .close');
    $(userIcon).on('click', showPopup);
    $(closeIcon).on('click', hidePopup);
  }

  // ---------------------
  // Side Navigation Panel
  // ---------------------

  /**
   * Toggle accordion item handler.
   * @param {MouseEvent} evt
   * @return {boolean}
   */
  function toggleMenuItem(evt) {
    var item = $(evt.currentTarget);
    var itemT = $(evt.target);
    if ($(itemT).prop('tagName').toLowerCase() === 'a') {
      this.stopPropagation();
      return true;
    } else if (item.prop('tagName').toLowerCase() !== 'li') {
      return true;
    }
    item.toggleClass('active');

    if (item.attr('aria-expanded') === 'true') {
      item.removeAttr('aria-expanded');
    } else {
      item.attr('aria-expanded', 'true');
    }
    return false;
  }

  /**
   * Listen and handle side nav close transition.
   * To hide the side nav when the animation is complete.
   */
  function listenNavTransition() {
    $('#mainNav').one('webkitTransitionEnd otransitionend oTransitionEnd msTransitionEnd transitionend', function (evt) {
      // Hide masthead to avoid selecting items with tab key when the nav is closed
      if (evt.originalEvent.propertyName === 'transform') {
        if ($(this).hasClass('shown')) {
          $(this).removeClass('hidden');
        } else {
          $(this).addClass('hidden');
        }
      }
    });
  }

  /**
   * Show left-side navigation.
   * Sliding animation is CSS's responsability.
   */
  function showNavigation() {
    var navOpenButton = $(document.querySelector('#sectionsmenu'));
    var mainNav = document.querySelector('#mainNav');
    var jqmainNav = $(mainNav);

    if (!jqmainNav.hasClass('shown')) {
      navOpenButton.attr('aria-expanded', 'true');
      jqmainNav.removeClass('hidden');
      jqmainNav.outerHeight(); // Trigger a reflow for display to take effect
      jqmainNav.addClass('shown');
      jqmainNav.removeAttr('aria-hidden');
      document.querySelector('#mainNav button:first-child').focus();
    }
  }

  function closeNavigation() {
    var navOpenButton = $(document.querySelector('#sectionsmenu'));
    var mainNav = document.querySelector('#mainNav');
    var jqmainNav = $(mainNav);
    listenNavTransition();

    if (jqmainNav.hasClass('shown')) {
      jqmainNav.removeClass('shown');
      jqmainNav.attr('aria-hidden', 'true');
      navOpenButton.removeAttr('aria-expanded');
    }
  }

  /**
   * Shows search box input when search is in side navigation.
   * Transitions to 'open' state as named by UI design.
   */
  function showNavSearch() {
    var searchbox = document.querySelector('.main-nav form.searchbox');
    $(searchbox).addClass('visible');
    $(searchbox.querySelector('input[type=text]')).focus();
    $(this).addClass('hidden');
  }

  /**
   * Hides the search box input and shows the icon and text.
   * Transitions to 'closed' state as named by UI design.
   */
  function hideNavSearch() {
    var searchbox = document.querySelector('.main-nav form.searchbox');
    $(searchbox).removeClass('visible');
    $('.main-nav .search-placeholder').removeClass('hidden');
  }

  /**
   * Handler for focus out of side navigation.
   * @param {FocusEvent} evt
   * @return {boolean}
   */
  function focusOutOfNav(evt) {
    var mainNav = this;
    var isContained = mainNav.contains(evt.relatedTarget);

    if (!isContained) {
      closeNavigation();
      hideNavSearch();
    }

    return isContained;
  }

  /**
   * Attach handlers for accordion behavior.
   * Navigation display, hide and focus transitions.
   */
  function setupNavigation() {
    var isReady = $('.main-nav').hasClass('jsready');
    if (isReady) {
      return;
    }
    var nav = document.querySelectorAll('.main-nav .nav-item.expandable, .main-nav .nav-item.expandable span');
    nav.forEach(function (navitem) {
      $(navitem).on('click', toggleMenuItem);
    });

    $('.subcategory-title').on('click', function (evt) {
      evt.preventDefault();
      evt.stopPropagation();
      $(this).css({ outline: 'none' });
    });

    // Open/Close menu
    var navOpenButton = document.querySelector('#sectionsmenu');
    var navCloseButton = document.querySelector('#closenavigation');
    $(navOpenButton).on('click', showNavigation);
    $(navCloseButton).on('click', closeNavigation);
    $('.main-nav').addClass('jsready')
      .on('focusout', focusOutOfNav);

    // Open/Close search in navigation
    $('.main-nav .search-placeholder').click(showNavSearch);
  }

  // Show the second level of the menu for niche publications
  function quickNavSecondLevel() {
    $('.top-navigation li ul').hide().removeClass('quick-nav-second-level');
    if(!('ontouchstart' in window)) {
      $('.top-navigation .item-first-level')
        .hover(function () {
          $('ul', this)
            .addClass('expanded-submenu')
            .slideDown(400);
        }, function () {
          $('ul', this)
            .stop()
            .slideUp(200, function () {
              $(this)
                .removeClass('expanded-submenu');
            });
        });
    }
  }

  // ---------------------
  // Misc
  // ---------------------

  /**
   * For accessibility and good aesthetics the focused outline
   * is not shown when the interaction is done via mouse.
   * When tab or arrow keys are used the outline is enabled via CSS.
   */
  function accessibleOutline() {
    $('body').on('click', function () {
      $(this).addClass('mouse');
    });

    $('body').on('keydown', function (evt) {
      var key = evt.which;
      // Enable outline for tab and arrow keys
      if (key === 9 || (key >= 37 && key <= 40)) {
        $(this).removeClass('mouse');
      }
    });
  }

  function init() {
    setupSearch();
    setupLogin();
    setupNavigation();
    accessibleOutline();
    quickNavSecondLevel();
  }

  return {
    // eslint-disable-next-line comma-dangle
    init: init
  };
}());

masthead.init();


/***/ }),
/* 5 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(4);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 6 */
/***/ (function(module, exports) {

/* global $ */
const extraShareOptions = (function () {
  const init = () => {
    function desktopEvent() {
      $('.event-author-section').hover(() => {
        $('.extra-share-author-section').show();
      }, () => {
        $('.extra-share-author-section').hide();
      });
      $('.event-read-next-section').hover(() => {
        $('.extra-share-read-next-section').show();
      }, () => {
        $('.extra-share-read-next-section').hide();
      });
    }

    function toggleBubble(elementClass) {
      if ('ontouchstart' in document.documentElement) {
        $(elementClass).toggle();
      }
    }

    desktopEvent();

    $('.event-author-section').click(() => {
      toggleBubble('.extra-share-author-section');
    });

    $('.event-read-next-section').click(() => {
      toggleBubble('.extra-share-read-next-section');
    });
  };
  return {
    init,
  };
}());

extraShareOptions.init();


/***/ }),
/* 7 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(6);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 8 */
/***/ (function(module, exports) {

// /*
// /!* global $ window document *!/
// const newsletterCta = (function (isNewsletterEnabled) {
//     const $subscription = document.querySelector('#newsletter .subscription-cta-widget');
//     const $newsletter = document.querySelector('#newsletter .newsletter-cta-macro');
//     const listNames = mi.pageInfo.getConf('newsletterPreferenceListNames');
//     const newsletterData = {
//         siteName: mi.pageInfo.getConf('newsletterSiteName')
//     };
//
//     const $email = $newsletter.querySelector('.cta-input-email input[type="text"]');
//     const $submitBtn = $newsletter.querySelector('.more-link-macro a');
//
//     const isEmailValid = (email) => {
//         var re = /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
//         return re.test(email);
//     };
//
//     const getRandomArbitrary = (min, max) => {
//         const randomNum = Math.random() * ((max - min) + min);
//         return Math.round(randomNum);
//     };
//
//     const removeNode = (node) => {
//         if (node) {
//             node.parentNode.removeChild(node);
//         }
//     };
//
//     const insertAfterNode = (el, node) => {
//         if (node) {
//             node.parentNode.insertBefore(el, node.nextSibling);
//         }
//     };
//
//     const insertContent = (pos) => {
//         const $title = document.createElement('h3');
//         $title.innerHTML = listNames[pos].title;
//         $newsletter.insertBefore($title, $newsletter.firstChild);
//
//         const $subtitle = document.createElement('p');
//         $subtitle.innerHTML = listNames[pos].headline;
//         insertAfterNode($subtitle, $title);
//     };
//
//     const displayRandomNewsletter = (num) => {
//         if($subscription) {
//             removeNode($subscription);
//         }
//         $newsletter.classList.remove('hidden');
//         newsletterData.preferenceListName = listNames[num].name;
//         $newsletter.querySelector('#newsletterType').value = listNames[num].name;
//         insertContent(num);
//         disableSubmitBtn();
//         eventHandlers();
//     };
//
//     const removeRandomCTA = () => {
//         if (isNewsletterEnabled === true) {
//             const num = getRandomArbitrary(0, listNames.length);
//             if ($subscription && num === 0) {
//                 removeNode($newsletter);
//                 notifyWidgetSelected($subscription);
//                 $subscription.classList.remove('hidden');
//             } else {
//                 displayRandomNewsletter(num - 1)
//                 notifyWidgetSelected($newsletter);
//             }
//         }
//         else {
//             const num = getRandomArbitrary(0, 1);
//             if (num === 1) {
//                 removeNode($newsletter);
//                 notifyWidgetSelected($subscription);
//                 $subscription.classList.remove('hidden');
//             } else {
//                 removeNode($subscription);
//                 notifyWidgetSelected($newsletter);
//                 $newsletter.classList.remove('hidden')
//             }
//         }
//     };
//
//     const notifyWidgetSelected = (widget) => {
//         const newEvent = new CustomEvent('cta_widget', {detail: widget});
//         window.dispatchEvent(newEvent);
//     };
//
//     const enableSubmitBtn = () => {
//         $submitBtn.classList.remove('disabled');
//     };
//
//     const disableSubmitBtn = () => {
//         $submitBtn.classList.add('disabled');
//     };
//
//     const showErrorMsg = (node, msg) => {
//         const div = document.createElement('div');
//         div.classList.add('errorText');
//         div.innerHTML = msg;
//         insertAfterNode(div, node);
//     };
//
//     const validateNewsletter = () => {
//         const errorMsg = $newsletter.querySelector('.errorText');
//         if (errorMsg) {
//             errorMsg.remove();
//         }
//         if ($email.classList.contains('error')) {
//             $email.classList.remove('error');
//         }
//         if (!isEmailValid($email.value)) {
//             showErrorMsg($email, "Please insert a valid email address!");
//             $email.classList.add('error');
//             disableSubmitBtn();
//         } else {
//             enableSubmitBtn();
//         }
//     };
//
//     const submitNewsletter = (e) => {
//         e.preventDefault();
//
//         validateNewsletter();
//
//         if ($submitBtn.classList.contains('disabled')) {
//             return false;
//         }
//         const form = $newsletter;
//         grecaptcha.execute();
//         newsletterData.emailAddress = $email.value;
//         $newsletter.setAttribute('data-json', JSON.stringify(newsletterData));
//     };
//
//     const eventHandlers = (e) => {
//         $email.addEventListener('change', function (e) {
//             validateNewsletter();
//         });
//         $newsletter.addEventListener('submit', function (e) {
//             submitNewsletter(e);
//         });
//         $submitBtn.addEventListener('click', function (e) {
//             submitNewsletter(e);
//         });
//     };
//
//     const init = () => {
//         /!* eslint-disable no-undef *!/
//         const isLoggedIn = getSubscribedFlag;
//         /!* eslint-enable no-undef *!/
//
//         if (isLoggedIn) {
//             const num = getRandomArbitrary(0, listNames.length - 1);
//             displayRandomNewsletter(num);
//         } else {
//             removeRandomCTA();
//         }
//
//     };
//
//     return {
//         init
//     };
// }(mi.pageInfo.getConf('isNewsletterEnabled')));
//
// newsletterCta.init();*/


/***/ }),
/* 9 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(8);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 10 */
/***/ (function(module, exports) {

/* global $ */
const breakingNews = (function () {
  const init = () => {
    $('.close-breaking-news').click(() => {
      $('.breaking-news-macro').slideUp('slow', () => {
        $('.breaking-news-macro').attr('style', 'display: none !important');
      });
    });
  };

  return {
    init,
  };
}());

breakingNews.init();


/***/ }),
/* 11 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(10);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 12 */
/***/ (function(module, exports) {

/* global $ window document videojs */
const mediaComponentActions = (function () {
  const videoGalleryHandler = () => {
    $('.media-gallery .column').on('click', function () {
      const videoPlaying = $(this).siblings().find('.vjs-playing').find('video')
        .attr('id');

      const videoPlayingAd = $(this).siblings().find('.vjs-ad-playing').find('video')
        .attr('id');

      if (videoPlaying) {
        videojs(videoPlaying).pause();
      } else if (videoPlayingAd) {
        videojs.getPlayer(videoPlayingAd).ima.pauseAd();
      }
    });
  };

  const init = () => {
    videoGalleryHandler();
  };

  return {
    init,
  };
}());

mediaComponentActions.init();


/***/ }),
/* 13 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(12);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 14 */
/***/ (function(module, exports) {

/* global $ window document */
const mediaComponent = (function () {
  const itemClickListener = (e) => {
    const targetElement = $(e.target).closest('.column');

    if (!targetElement.hasClass('active')) {
      e.stopPropagation();
      e.preventDefault();
      targetElement.siblings('.active').removeClass('active');
      targetElement.addClass('active');
    }
  };

  const init = () => {
    const items = document.querySelectorAll('.media-gallery .column');

    $(window).on('resize load', () => {
      if (window.innerWidth > 1020) {
        items.forEach(item => $(item).on('click', itemClickListener));
      } else {
        items.forEach(item => $(item).off('click', itemClickListener));
      }
    });
  };

  return {
    init,
  };
}());

mediaComponent.init();


/***/ }),
/* 15 */
/***/ (function(module, exports, __webpack_require__) {

/* WEBPACK VAR INJECTION */(function(global) {module.exports = global["Util"] = __webpack_require__(14);
/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(1)))

/***/ }),
/* 16 */
/***/ (function(module, exports, __webpack_require__) {

__webpack_require__(15);
__webpack_require__(13);
__webpack_require__(11);
__webpack_require__(9);
__webpack_require__(7);
module.exports = __webpack_require__(5);


/***/ })
/******/ ]);