var mistats = mistats || {};

mistats.ContentTracker = function (pNumWords, pPrepend, pTrackTag)
{
   var docBody;
   var clip;
   var suffix;
   var COOKIE_NAME;

   COOKIE_NAME = 'micpytrk=';

   suffix = pPrepend + location.href + pTrackTag;
   docBody = document.getElementsByTagName('body');

   if (docBody.length)
      docBody = docBody[0];
   else
      return null;

   addLinkToSelection = function (pEvent)
   {
      var r;
      var theSel;
      var thisRange;
      var startNode;
      var startIdx;
      var endNode;
      var endIdx;
      var origText;

      if (typeof window.getSelection === 'function')
      {
         theSel = window.getSelection();
         if (!theSel.rangeCount)
            return true;

         thisRange = '';
         for (r = 0; r < theSel.rangeCount; r++)
            thisRange += theSel.getRangeAt(r).toString();

         if (thisRange.match(/\w+/g).length < pNumWords)
            return;

         if (!clip)
         {
            clip = document.createElement('div');
            clip.style.width = '1px';
            clip.style.height = '1px';
            clip.style.color = '#000000';
            clip.style.font = 'normal normal normal 10pt sans-serif';
            clip.style.textAlign = 'left';
            clip.style.textTransform = 'none';
            clip.style.overflow = 'hidden';
         }

         clip.innerHTML = '<br />' + suffix;

         if (!theSel.removeRange || (!theSel.modify && theSel.rangeCount === 1))
         {
            thisRange = theSel.getRangeAt(0).cloneRange();
            startNode = thisRange.startContainer;
            startIdx = thisRange.startOffset;
            endNode = thisRange.endContainer;
            endIdx = thisRange.endOffset;

            theSel.removeAllRanges();

            if (endNode.nodeName === '#text')
            {
               origText = endNode.data;
               endNode.data = endNode.data.substring(0, endIdx);
            }

            clip = endNode.parentNode.insertBefore(clip, endNode.nextSibling);
            thisRange.setStart(startNode, startIdx);
            thisRange.setEnd(clip, clip.childNodes.length);
            theSel.addRange(thisRange);

            setTimeout(function ()
            {
               theSel.removeAllRanges();
               clip.parentNode.removeChild(clip);
               if (origText)
                  endNode.data = origText;
               thisRange.setStart(startNode, startIdx);
               thisRange.setEnd(endNode, endIdx);
               theSel.addRange(thisRange);
            }, 0);
         } else
         {
            docBody.appendChild(clip);
            thisRange = document.createRange();
            thisRange.selectNodeContents(clip);
            theSel.addRange(thisRange);
            
            setTimeout(function ()
            {
               docBody.removeChild(clip);
            }, 0);
         }
      } else if (typeof document.selection === 'object' && typeof window.clipboardData === 'object')
      {
         theSel = document.selection.createRange().text;

         if (theSel.match(/\w+/g).length < pNumWords)
            return true;

         window.clipboardData.setData('Text', theSel + '\n\n' + suffix);
         pEvent.returnValue = false;

         return false;
      }

      return true;
   };

   function writeTrackerCookie(pDisable)
   {
      var date;

      date = new Date();
      date.setTime(date.getTime() + (((pDisable) ? 60 : -1) * 86400000));

      document.cookie = COOKIE_NAME + ((pDisable) ? '1' : '') + '; expires=' + date.toGMTString() + '; path=/';

      return pDisable;
   };

   function readTrackerCookie()
   {
      var c;
      var crumbs;

      crumbs = document.cookie.split('; ');
      for (c = 0; c < crumbs.length; c++)
         if (crumbs[c].match(new RegExp(COOKIE_NAME)))
            if (crumbs[c].split(COOKIE_NAME)[1] === '1')
               return writeTrackerCookie(true);

      return false;
   };

   function trackerDisabled()
   {
      if (location.search.match(/disableTracer=on/i))
         return writeTrackerCookie(true);
      else if (location.search.match(/disableTracer=off/i))
         return writeTrackerCookie(false);

      return readTrackerCookie();
   };

   if (!trackerDisabled())
   {
      if (docBody.attachEvent)
         docBody.attachEvent('oncopy', addLinkToSelection);
      else if (window.addEventListener)
         window.addEventListener('copy', addLinkToSelection, false);
   }

   return this;
}

mistats.contentTracker = new mistats.ContentTracker(12, 'Read more here: ', '#storylink=cpy');

