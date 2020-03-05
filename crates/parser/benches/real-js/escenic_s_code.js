var s_account;
var s;

s_account = mistats.account + ',mccltAllMcClatchy'; 
s = s_gi(s_account);

s.currencyCode = 'USD';

s.trackDownloadLinks = false;
s.trackExternalLinks = false;
s.trackInlineStats = true;
s.linkDownloadFileTypes = 'exe,zip,wav,mp3,mov,mpg,avi,wmv,doc,pdf,xls,xlsx';
s.linkInternalFilters = 'javascript:,.'; 
s.linkLeaveQueryString = false;

s.referrer = '';
s.pageURL = '';

s.usePlugins = true;

mistats.clearList = {};

mistats.setOnce = function (pName, pVal)
{
   mistats.clearList[pName] = true;
   s[pName] = (typeof pVal === 'undefined' ? s[pName] : pVal) || '';
};

mistats.score = function (pAct)
{
   var act;
   var cs;

   act =
   {
      'search':     {name: 'Internal Search',           value: 4},
      'story':      {name: 'Story Page View',           value: 1},
      'comment':    {name: 'Post Comment',              value: 4},
      'social':     {name: 'Social Media Interactions', value: 5},
      'login':      {name: 'Login Successful',          value: 2},
      'video':      {name: 'Completed Video',           value: 3},
      'download':   {name: 'File Download',             value: 1},
      'tool':       {name: 'Site Tool',                 value: 5},
      'newsletter': {name: 'Newsletter Signup',         value: 4},
      'appinstall': {name: 'Install App',               value: 4},
      'promoclick': {name: 'Promo Banner Click',        value: 3}
   }[pAct || ''];

   if (!act)
      return;

   cs = (s.eVar63 || '+0').substr(1);
   cs = (isNaN(cs) ? 0 : parseInt(cs)) + act.value;

   mistats.events.add('event44=' + cs);
   mistats.setOnce('eVar63', '+' + cs);
   mistats.setOnce('eVar65', '+' + cs);

   mistats.listProp.add('eVar64', act.name, true);
   mistats.setOnce('eVar64');
   mistats.setOnce('prop38', 'D=v64');
};

mistats.failover = mistats.failover ||
{
   init:    false,
   success: false,

   beacons:  [],
   timeouts: [],

   base: function ()
   {
      var d;
      var i;
      var o;
      var u;

      if (mistats.failsafe && mistats.failsafe.pref)
         return '//' + mistats.failsafe.pref + '/ng.gif?';

      o = document.getElementsByTagName('script');

      for (i = 0; !u && i < o.length; i++)
         u = (o[i].src || '').match(/^https?:\/\/media\d?\.[^\.]+\.com\/mi\w+/i);

      if (!u)
         return '';

      u = u[0];
      d = u.replace(/^https?:\/\/media\d?\./, '').replace(/\.com\/.+/, '');

      return u.replace(/\.com\/.+/, '.com/' + d + '/ng.gif?');
   }(),

   clear: function ()
   {
      mistats.failover.success = true;
      mistats.failover.beacons = [];
      while (mistats.failover.timeouts.length)
         clearTimeout(mistats.failover.timeouts.shift());
      window.removeEventListener((mistats.aborted ? 'unload' : ('onpagehide' in window ? 'pagehide' : 'beforeunload')), mistats.failover.track, false);
   },

   track: function (pEvent)
   {
      var b;
      var f;

      if (mistats.failover.success || !mistats.failover.beacons.length || !mistats.failover.base)
         return mistats.failover.clear();

      b = (mistats.failover.beacons.shift() || '').replace(/.*\.net\/b\/ss/, '');

      if (!b)
         return;

      f = new Image();
      f.src = mistats.failover.base + b + '&UA=' + encodeURIComponent(navigator.userAgent).substr(0, 2083);

      window.console && console.log && (console.log('mistats request failover'));
      pEvent && (mistats.failover.track(pEvent));
   },

   add: function (pUrl)
   {
      var i;

      if (!pUrl)
         return;

      for (i = 0; i < mistats.failover.beacons.length; i++)
         if (mistats.failover.beacons[i] === pUrl)
            return;

      mistats.failover.beacons[mistats.failover.beacons.length] = pUrl;
      mistats.failover.timeouts[mistats.failover.timeouts.length] = setTimeout(mistats.failover.track, 5000);

      if (mistats.failover.init)
         return;

      mistats.failover.init = true;
      window.addEventListener((mistats.aborted ? 'unload' : ('onpagehide' in window ? 'pagehide' : 'beforeunload')), mistats.failover.track, false);
   }
};

s.registerPreTrackCallback(function (pUrl)
{
   var i;

   window.console && console.log && (console.log('mistats request sent'));

   s.prop37 = '';
   s.events = '';
   s.products = '';
   s.campaign = '';

   for (i in mistats.clearList)
      s[i] = '';
   mistats.clearList = {};

   pUrl && !(pUrl || '').match(/&pev\d=|&pe=/) && mistats.escenicId && mistats.updateStoryHistory && (mistats.updateStoryHistory(pUrl));
   pUrl && !mistats.failover.success && (mistats.failover.add(pUrl));
});

s.registerPostTrackCallback(function (pUrl)
{
   mistats.failover.success = true;
   mistats.failover.clear();
   window.console && console.log && (console.log('mistats request succeeded'));
});

function s_doPlugins(s)
{
   var custIds;
   var i;
   var mcid;
   var obj;
   var year;

   year = (new Date()).getFullYear();

   s.prop11 = s.prop11 || '';

   s.prop11 = s.prop11.replace(/micb:[a-z]{2,3}\|/g, '');
   s.prop11 = 'micb:' + (mistats.adobe ? 'yes' : 'no') + '|' + s.prop11;

   s.prop11 = s.prop11.replace(/ecidtimeout:[a-z]{2,3}\|/g, '');
   s.prop11 = 'ecidtimeout:' + (mistats.ecidTimeout ? 'yes' : 'no') + '|' + s.prop11;

   s.prop11 = s.prop11.replace(/ecidfailed:[a-z]{2,3}\|/g, '');
   s.prop11 = 'ecidfailed:' + (mistats.ecidFailed ? 'yes' : 'no') + '|' + s.prop11;

   s.prop33 = s.getTimeParting('h', '-5', year); 
   s.prop34 = s.getTimeParting('d', '-5', year); 

   if (mistats.noSlots)
   {
      s.eVar33 = 'adblock:yes';
      s.c_w('mi_nas', '1', (new Date((new Date()).getTime() + 1800000)));
   }

   s.eVar15 = s.getNewRepeat(30, 'mi_nr');
	s.eVar55 = s.getPreviousValue(s.eVar4, 'mi_ppn');
   s.eVar71 = s.getVisitNum(30, 'mi_s_vnmn', 'mi_iv');

   s.eVar25 = mistats.categoryAffinity || '';

   if (mistats.subData)
   {
      s.eVar67 = mistats.subData.getStatus();
      s.eVar75 = mistats.subData.getService();
      s.eVar76 = mistats.subData.getExpires();
   }

   if (s.eVar55)
      s.prop44 = s.c_r('mi_ppv') || 'No View Data';
   else
   {
      s.eVar55 = 'Entry Page';
      s.prop44 = 'Entry Page';
   }

   if (mistats.atResponse && mistats.atResponse.responseTokens && mistats.atResponse.responseTokens.length)
   {
      obj = mistats.atResponse.responseTokens[mistats.atResponse.responseTokens.length - 1];
      obj && obj['activity.name'] && obj['experience.name'] && (s.eVar48 = obj['activity.name'] + ': ' + obj['experience.name']);
   }

   s.plugins = '';

   s.AudienceManagement.setup(
   {
      'partner': 'mcclatchy',
      'containerNSID': 0,
      'uuidCookie':
      {
         'name': 'aam_uuid',
         'days': 30
      }
   });

   if (s.visitor && s.visitor.setCustomerIDs)
   {
      mcid = s.visitor.getMarketingCloudVisitorID() || '';
      mcid && (custIds = {'mcid': mcid});

      if (mistats.subscriptions && mistats.subscriptions.ucid)
      {
         custIds = custIds || {};
         custIds['ucid'] =
         {
            'id': mistats.subscriptions.ucid,
            'authState': mistats.subscriptions.id ? (mistats.subscriptions.loggedIn ? 1 : 2) : 0
         };
         custIds['AdobeCampaignID'] =
         {
            'id': mistats.subscriptions.ucid,
            'authState': mistats.subscriptions.id ? (mistats.subscriptions.loggedIn ? 1 : 2) : 0
         };
      }
      custIds && (s.visitor.setCustomerIDs(custIds));
   }
/*
   if (s.visitor && s.visitor.setCustomerIDs && mistats.subscriptions && mistats.subscriptions.ucid)
      s.visitor.setCustomerIDs(
      {
         'ucid':
         {
            'id': mistats.subscriptions.ucid,
            'authState': mistats.subscriptions.id ? (mistats.subscriptions.loggedIn ? 1 : 2) : 0
         },
         'AdobeCampaignID':
         {
            'id': mistats.subscriptions.ucid,
            'authState': mistats.subscriptions.id ? (mistats.subscriptions.loggedIn ? 1 : 2) : 0
         }
      });
*/
   obj = {};
   for (i in s)
      if (typeof s[i] === 'string')
      {
         if (i.match(/^((eVar|hier)\d+|pageURL|referrer|campaign)$/))
            s[i] = mistats.removeDiacritics((s[i] || '').replace(/<[^>]+>/g, '').replace(/^\s+/, '').replace(/\s+$/, '').replace(/\s+/g, ' ')).substr(0, 255);
         else if (i.match(/^(pageName|channel|prop\d+)$/))
            s[i] = mistats.removeDiacritics((s[i] || '').replace(/<[^>]+>/g, '').replace(/^\s+/, '').replace(/\s+$/, '').replace(/\s+/g, ' ')).substr(0, 100);
         obj[i] = s[i];
      }

   if (mistats.bizunit === 'WIC')
      setTimeout(function ()
      {
         if (typeof AWS !== 'undefined' && mistats.kinesis && mistats.kinesis.putRecords)
            try
            {
               mistats.kinesis.putRecords(
               {
                  Records:
                  [
                     {
                        Data: JSON.stringify(obj),
                        PartitionKey: 'partition-' + AWS.config.credentials.identityId
                     }
                  ],
                  StreamName: 'MontyDeleteMe'
               }, function (pError, pData)
               {
                  if (pError)
                     console.log('kinesis_putrecords_error');
               });
            } catch (error)
            {
               if (pError)
                  console.log('kinesis_putrecords_error');
            }
      }, 0);
};

s.doPlugins = s_doPlugins;

mistats.removeDiacritics = function (pStr)
{
   var i;
   var map;

   map =
   [
      {base: '&',  letters: /&amp;/g},
      {base: '-',  letters: /[\u2012\u2013\u2014\u2015]/g},
      {base: '',   letters: /[\u2028\u2029]/g},
      {base: '',   letters: /[\u00BF\u00A1]/g},
      {base: '"',  letters: /[\u201C\u201D]/g},
      {base: '"',  letters: /&quot;/g},
      {base: '"',  letters: /&#34;/g},
      {base: '\'', letters: /[\u2018\u2019]/g},
      {base: '\'', letters: /&#39;/g},
      {base: 'A',  letters: /[\u0041\u24B6\uFF21\u00C0\u00C1\u00C2\u1EA6\u1EA4\u1EAA\u1EA8\u00C3\u0100\u0102\u1EB0\u1EAE\u1EB4\u1EB2\u0226\u01E0\u00C4\u01DE\u1EA2\u00C5\u01FA\u01CD\u0200\u0202\u1EA0\u1EAC\u1EB6\u1E00\u0104\u023A\u2C6F]/g},
      {base: 'AA', letters: /[\uA732]/g},
      {base: 'AE', letters: /[\u00C6\u01FC\u01E2]/g},
      {base: 'AO', letters: /[\uA734]/g},
      {base: 'AU', letters: /[\uA736]/g},
      {base: 'AV', letters: /[\uA738\uA73A]/g},
      {base: 'AY', letters: /[\uA73C]/g},
      {base: 'B',  letters: /[\u0042\u24B7\uFF22\u1E02\u1E04\u1E06\u0243\u0182\u0181]/g},
      {base: 'C',  letters: /[\u0043\u24B8\uFF23\u0106\u0108\u010A\u010C\u00C7\u1E08\u0187\u023B\uA73E]/g},
      {base: 'D',  letters: /[\u0044\u24B9\uFF24\u1E0A\u010E\u1E0C\u1E10\u1E12\u1E0E\u0110\u018B\u018A\u0189\uA779]/g},
      {base: 'DZ', letters: /[\u01F1\u01C4]/g},
      {base: 'Dz', letters: /[\u01F2\u01C5]/g},
      {base: 'E',  letters: /[\u0045\u24BA\uFF25\u00C8\u00C9\u00CA\u1EC0\u1EBE\u1EC4\u1EC2\u1EBC\u0112\u1E14\u1E16\u0114\u0116\u00CB\u1EBA\u011A\u0204\u0206\u1EB8\u1EC6\u0228\u1E1C\u0118\u1E18\u1E1A\u0190\u018E]/g},
      {base: 'F',  letters: /[\u0046\u24BB\uFF26\u1E1E\u0191\uA77B]/g},
      {base: 'G',  letters: /[\u0047\u24BC\uFF27\u01F4\u011C\u1E20\u011E\u0120\u01E6\u0122\u01E4\u0193\uA7A0\uA77D\uA77E]/g},
      {base: 'H',  letters: /[\u0048\u24BD\uFF28\u0124\u1E22\u1E26\u021E\u1E24\u1E28\u1E2A\u0126\u2C67\u2C75\uA78D]/g},
      {base: 'I',  letters: /[\u0049\u24BE\uFF29\u00CC\u00CD\u00CE\u0128\u012A\u012C\u0130\u00CF\u1E2E\u1EC8\u01CF\u0208\u020A\u1ECA\u012E\u1E2C\u0197]/g},
      {base: 'J',  letters: /[\u004A\u24BF\uFF2A\u0134\u0248]/g},
      {base: 'K',  letters: /[\u004B\u24C0\uFF2B\u1E30\u01E8\u1E32\u0136\u1E34\u0198\u2C69\uA740\uA742\uA744\uA7A2]/g},
      {base: 'L',  letters: /[\u004C\u24C1\uFF2C\u013F\u0139\u013D\u1E36\u1E38\u013B\u1E3C\u1E3A\u0141\u023D\u2C62\u2C60\uA748\uA746\uA780]/g},
      {base: 'LJ', letters: /[\u01C7]/g},
      {base: 'Lj', letters: /[\u01C8]/g},
      {base: 'M',  letters: /[\u004D\u24C2\uFF2D\u1E3E\u1E40\u1E42\u2C6E\u019C]/g},
      {base: 'N',  letters: /[\u004E\u24C3\uFF2E\u01F8\u0143\u00D1\u1E44\u0147\u1E46\u0145\u1E4A\u1E48\u0220\u019D\uA790\uA7A4]/g},
      {base: 'NJ', letters: /[\u01CA]/g},
      {base: 'Nj', letters: /[\u01CB]/g},
      {base: 'O',  letters: /[\u004F\u24C4\uFF2F\u00D2\u00D3\u00D4\u1ED2\u1ED0\u1ED6\u1ED4\u00D5\u1E4C\u022C\u1E4E\u014C\u1E50\u1E52\u014E\u022E\u0230\u00D6\u022A\u1ECE\u0150\u01D1\u020C\u020E\u01A0\u1EDC\u1EDA\u1EE0\u1EDE\u1EE2\u1ECC\u1ED8\u01EA\u01EC\u00D8\u01FE\u0186\u019F\uA74A\uA74C]/g},
      {base: 'OI', letters: /[\u01A2]/g},
      {base: 'OO', letters: /[\uA74E]/g},
      {base: 'OU', letters: /[\u0222]/g},
      {base: 'P',  letters: /[\u0050\u24C5\uFF30\u1E54\u1E56\u01A4\u2C63\uA750\uA752\uA754]/g},
      {base: 'Q',  letters: /[\u0051\u24C6\uFF31\uA756\uA758\u024A]/g},
      {base: 'R',  letters: /[\u0052\u24C7\uFF32\u0154\u1E58\u0158\u0210\u0212\u1E5A\u1E5C\u0156\u1E5E\u024C\u2C64\uA75A\uA7A6\uA782]/g},
      {base: 'S',  letters: /[\u0053\u24C8\uFF33\u1E9E\u015A\u1E64\u015C\u1E60\u0160\u1E66\u1E62\u1E68\u0218\u015E\u2C7E\uA7A8\uA784]/g},
      {base: 'T',  letters: /[\u0054\u24C9\uFF34\u1E6A\u0164\u1E6C\u021A\u0162\u1E70\u1E6E\u0166\u01AC\u01AE\u023E\uA786]/g},
      {base: 'TZ', letters: /[\uA728]/g},
      {base: 'U',  letters: /[\u0055\u24CA\uFF35\u00D9\u00DA\u00DB\u0168\u1E78\u016A\u1E7A\u016C\u00DC\u01DB\u01D7\u01D5\u01D9\u1EE6\u016E\u0170\u01D3\u0214\u0216\u01AF\u1EEA\u1EE8\u1EEE\u1EEC\u1EF0\u1EE4\u1E72\u0172\u1E76\u1E74\u0244]/g},
      {base: 'V',  letters: /[\u0056\u24CB\uFF36\u1E7C\u1E7E\u01B2\uA75E\u0245]/g},
      {base: 'VY', letters: /[\uA760]/g},
      {base: 'W',  letters: /[\u0057\u24CC\uFF37\u1E80\u1E82\u0174\u1E86\u1E84\u1E88\u2C72]/g},
      {base: 'X',  letters: /[\u0058\u24CD\uFF38\u1E8A\u1E8C]/g},
      {base: 'Y',  letters: /[\u0059\u24CE\uFF39\u1EF2\u00DD\u0176\u1EF8\u0232\u1E8E\u0178\u1EF6\u1EF4\u01B3\u024E\u1EFE]/g},
      {base: 'Z',  letters: /[\u005A\u24CF\uFF3A\u0179\u1E90\u017B\u017D\u1E92\u1E94\u01B5\u0224\u2C7F\u2C6B\uA762]/g},
      {base: 'a',  letters: /[\u0061\u24D0\uFF41\u1E9A\u00E0\u00E1\u00E2\u1EA7\u1EA5\u1EAB\u1EA9\u00E3\u0101\u0103\u1EB1\u1EAF\u1EB5\u1EB3\u0227\u01E1\u00E4\u01DF\u1EA3\u00E5\u01FB\u01CE\u0201\u0203\u1EA1\u1EAD\u1EB7\u1E01\u0105\u2C65\u0250]/g},
      {base: 'aa', letters: /[\uA733]/g},
      {base: 'ae', letters: /[\u00E6\u01FD\u01E3]/g},
      {base: 'ao', letters: /[\uA735]/g},
      {base: 'au', letters: /[\uA737]/g},
      {base: 'av', letters: /[\uA739\uA73B]/g},
      {base: 'ay', letters: /[\uA73D]/g},
      {base: 'b',  letters: /[\u0062\u24D1\uFF42\u1E03\u1E05\u1E07\u0180\u0183\u0253]/g},
      {base: 'c',  letters: /[\u0063\u24D2\uFF43\u0107\u0109\u010B\u010D\u00E7\u1E09\u0188\u023C\uA73F\u2184]/g},
      {base: 'd',  letters: /[\u0064\u24D3\uFF44\u1E0B\u010F\u1E0D\u1E11\u1E13\u1E0F\u0111\u018C\u0256\u0257\uA77A]/g},
      {base: 'dz', letters: /[\u01F3\u01C6]/g},
      {base: 'e',  letters: /[\u0065\u24D4\uFF45\u00E8\u00E9\u00EA\u1EC1\u1EBF\u1EC5\u1EC3\u1EBD\u0113\u1E15\u1E17\u0115\u0117\u00EB\u1EBB\u011B\u0205\u0207\u1EB9\u1EC7\u0229\u1E1D\u0119\u1E19\u1E1B\u0247\u025B\u01DD]/g},
      {base: 'f',  letters: /[\u0066\u24D5\uFF46\u1E1F\u0192\uA77C]/g},
      {base: 'g',  letters: /[\u0067\u24D6\uFF47\u01F5\u011D\u1E21\u011F\u0121\u01E7\u0123\u01E5\u0260\uA7A1\u1D79\uA77F]/g},
      {base: 'h',  letters: /[\u0068\u24D7\uFF48\u0125\u1E23\u1E27\u021F\u1E25\u1E29\u1E2B\u1E96\u0127\u2C68\u2C76\u0265]/g},
      {base: 'hv', letters: /[\u0195]/g},
      {base: 'i',  letters: /[\u0069\u24D8\uFF49\u00EC\u00ED\u00EE\u0129\u012B\u012D\u00EF\u1E2F\u1EC9\u01D0\u0209\u020B\u1ECB\u012F\u1E2D\u0268\u0131]/g},
      {base: 'j',  letters: /[\u006A\u24D9\uFF4A\u0135\u01F0\u0249]/g},
      {base: 'k',  letters: /[\u006B\u24DA\uFF4B\u1E31\u01E9\u1E33\u0137\u1E35\u0199\u2C6A\uA741\uA743\uA745\uA7A3]/g},
      {base: 'l',  letters: /[\u006C\u24DB\uFF4C\u0140\u013A\u013E\u1E37\u1E39\u013C\u1E3D\u1E3B\u017F\u0142\u019A\u026B\u2C61\uA749\uA781\uA747]/g},
      {base: 'lj', letters: /[\u01C9]/g},
      {base: 'm',  letters: /[\u006D\u24DC\uFF4D\u1E3F\u1E41\u1E43\u0271\u026F]/g},
      {base: 'n',  letters: /[\u006E\u24DD\uFF4E\u01F9\u0144\u00F1\u1E45\u0148\u1E47\u0146\u1E4B\u1E49\u019E\u0272\u0149\uA791\uA7A5]/g},
      {base: 'nj', letters: /[\u01CC]/g},
      {base: 'o',  letters: /[\u006F\u24DE\uFF4F\u00F2\u00F3\u00F4\u1ED3\u1ED1\u1ED7\u1ED5\u00F5\u1E4D\u022D\u1E4F\u014D\u1E51\u1E53\u014F\u022F\u0231\u00F6\u022B\u1ECF\u0151\u01D2\u020D\u020F\u01A1\u1EDD\u1EDB\u1EE1\u1EDF\u1EE3\u1ECD\u1ED9\u01EB\u01ED\u00F8\u01FF\u0254\uA74B\uA74D\u0275]/g},
      {base: 'oi', letters: /[\u01A3]/g},
      {base: 'ou', letters: /[\u0223]/g},
      {base: 'oo', letters: /[\uA74F]/g},
      {base: 'p',  letters: /[\u0070\u24DF\uFF50\u1E55\u1E57\u01A5\u1D7D\uA751\uA753\uA755]/g},
      {base: 'q',  letters: /[\u0071\u24E0\uFF51\u024B\uA757\uA759]/g},
      {base: 'r',  letters: /[\u0072\u24E1\uFF52\u0155\u1E59\u0159\u0211\u0213\u1E5B\u1E5D\u0157\u1E5F\u024D\u027D\uA75B\uA7A7\uA783]/g},
      {base: 's',  letters: /[\u0073\u24E2\uFF53\u00DF\u015B\u1E65\u015D\u1E61\u0161\u1E67\u1E63\u1E69\u0219\u015F\u023F\uA7A9\uA785\u1E9B]/g},
      {base: 't',  letters: /[\u0074\u24E3\uFF54\u1E6B\u1E97\u0165\u1E6D\u021B\u0163\u1E71\u1E6F\u0167\u01AD\u0288\u2C66\uA787]/g},
      {base: 'tz', letters: /[\uA729]/g},
      {base: 'u',  letters: /[\u0075\u24E4\uFF55\u00F9\u00FA\u00FB\u0169\u1E79\u016B\u1E7B\u016D\u00FC\u01DC\u01D8\u01D6\u01DA\u1EE7\u016F\u0171\u01D4\u0215\u0217\u01B0\u1EEB\u1EE9\u1EEF\u1EED\u1EF1\u1EE5\u1E73\u0173\u1E77\u1E75\u0289]/g},
      {base: 'v',  letters: /[\u0076\u24E5\uFF56\u1E7D\u1E7F\u028B\uA75F\u028C]/g},
      {base: 'vy', letters: /[\uA761]/g},
      {base: 'w',  letters: /[\u0077\u24E6\uFF57\u1E81\u1E83\u0175\u1E87\u1E85\u1E98\u1E89\u2C73]/g},
      {base: 'x',  letters: /[\u0078\u24E7\uFF58\u1E8B\u1E8D]/g},
      {base: 'y',  letters: /[\u0079\u24E8\uFF59\u1EF3\u00FD\u0177\u1EF9\u0233\u1E8F\u00FF\u1EF7\u1E99\u1EF5\u01B4\u024F\u1EFF]/g},
      {base: 'z',  letters: /[\u007A\u24E9\uFF5A\u017A\u1E91\u017C\u017E\u1E93\u1E95\u01B6\u0225\u0240\u2C6C\uA763]/g}
   ];

   for (i = 0; i < map.length; i++)
      pStr = pStr.replace(map[i].letters, map[i].base);

   return pStr;
};

mistats.tagMender = function ()
{
   var badTag;
   var i;
   var objs;
   var tag;
   var tagSrc;

   objs = document.getElementsByTagName('script');
   for (i = 0; !tag && i < objs.length; i++)
      (objs[i].innerHTML || '').match(/var\s+mistats\s*=\s*mistats/) && (tag = objs[i]);

   if (!tag)
      return;

   tagSrc = (tag.innerHTML || '').split('\n');
   for (i = 0; i < tagSrc.length; i++)
   {
      tagSrc[i] = tagSrc[i].replace(/^\s+/, '').replace(/\s+$/, '').replace(/\s+/g, ' ');
      tagSrc[i].length && !tagSrc[i].match(/^var\s+mistats|^mistats\.\w+\s*=\s*".*";$/) && (badTag = true);
   }

   if (!badTag)
      return;

   tagSrc = tagSrc.join(' ').replace(/[\u2028\u2029]/g, '');

   try
   {
      eval(tagSrc.replace(/\smistats\./g, 'window.mistats.'));
   } catch (tagErr)
   {
   }

   mistats.contentsource = 'Broken|' + (mistats.contentsource || '');
};

mistats.tagMender();

mistats.listProp =
{
   remove: function (pProp, pItem)
   {
      var i;
      var items;
      var list;

      if (!(pProp && pItem))
         return;

      items = (s[pProp] || '').split(',');
      for (i = 0; i < items.length; i++)
         items[i] == pItem && (items[i] = '');

      list = [];
      for (i = 0; i < items.length; i++)
         items[i].length && (list[list.length] = items[i]);

      while (list.join(',').length > 100)
         list.shift();

      s[pProp] = list.join(',');
   },

   add: function (pProp, pItem, pDups)
   {
      var i;
      var list;

      if (!(pProp && pItem))
         return;

      !pDups && (mistats.listProp.remove(pProp, pItem));

      list = s[pProp] ? (s[pProp] || '').split(',') : [];
      list[list.length] = pItem;

      while (list.join(',').length > 100)
         list.shift();

      s[pProp] = list.join(',');
   }
};

mistats.products =
{
   remove: function (pStr)
   {
      var i;
      var key;
      var name;
      var newProd;
      var oldProd;

      key = (pStr || '').split(';');
      if (!s.products || key.length < 4)
         return;

      key = key[0] + ';' + key[1];
      oldProd = s.products.split(',');
      for (i = 0; i < oldProd.length; i++)
      {
         name = oldProd[i].split(';');
         (name.length < 4 || key === (name[0] + ';' + name[1])) && (oldProd[i] = '');
      }

      newProd = [];
      for (i = 0; i < oldProd.length; i++)
         oldProd[i].length > 0 && (newProd[newProd.length] = oldProd[i]);

      s.products = newProd.join(',');
   },

   add: function (pStr)
   {
      var i;
      var prod;

      if (!pStr)
         return;

      if (!Array.isArray(pStr))
         pStr = [pStr];
      for (i = 0; i < pStr.length; i++)
      {
         mistats.products.remove(pStr[i]);
         prod = s.products ? s.products.split(',') : [];
         prod[prod.length] = pStr[i];
         s.products = prod.join(',');
      }
   }
};

mistats.events =
{
   remove: function (pStr)
   {
      var i;
      var key;
      var name;
      var newEvt;
      var oldEvt;

      key = (pStr || '').split('=');
      if (!(s.events && key[0]))
         return;

      oldEvt = s.events.split(',');
      for (i = 0; i < oldEvt.length; i++)
      {
         name = oldEvt[i].split('=');
         key[0] === name[0] && (oldEvt[i] = '');
      }

      newEvt = [];
      for (i = 0; i < oldEvt.length; i++)
         oldEvt[i].length > 0 && (newEvt[newEvt.length] = oldEvt[i]);

      s.events = newEvt.join(',');
   },

   add: function (pStr)
   {
      var events;
      var i;

      if (!pStr)
         return;

      if (!Array.isArray(pStr))
         pStr = [pStr];

      for (i = 0; i < pStr.length; i++)
         mistats.events.remove(pStr[i]);
      for (i = 0; i < pStr.length; i++)
      {
         events = s.events ? s.events.split(',') : [];
         events[events.length] = pStr[i];
         s.events = events.join(',');
      }
   }
};

mistats.unbind = mistats.unbind || function (pObj, pType, pCallout)
{
   if (!pObj)
      return;

   if (pObj.removeEventListener)
      pObj.removeEventListener(pType, pCallout, false);
   else if (pObj.detachEvent)
      pObj.detachEvent('on' + pType, pCallout);
};

mistats.bind = mistats.bind || function (pObj, pType, pCallout)
{
   if (!pObj)
      return;

   if (pObj.addEventListener)
      pObj.addEventListener(pType, pCallout, false);
   else if (pObj.attachEvent)
      pObj.attachEvent('on' + pType, pCallout);
};

mistats.getElementsByClassName = mistats.getElementsByClassName || function (pClass, pParent)
{
   var a;
   var allObjs;
   var matches;

   matches = [];

   if (typeof pParent === 'string')
      pParent = document.getElementById(pParent);

   if (!pParent)
      pParent = document;

   if (typeof pClass === 'string')
      pClass = new RegExp('^' + pClass + '$');

   allObjs = pParent.getElementsByTagName('*');

   for (a = 0; a < allObjs.length; a++)
      if (pClass.test(allObjs[a].className || ''))
         matches[matches.length] = allObjs[a];

   return matches;
};

mistats.getElementByClassName = mistats.getElementByClassName || function (pClass, pParent)
{
   var e;

   e = mistats.getElementsByClassName(pClass, pParent);
   if (e.length)
      return e[0];

   return null;
};

/************************** PLUGINS SECTION *************************/
/* You may insert any plugins you wish to use here.                 */

/*
* Utility Function: split v1.5 (JS 1.0 compatible)
*/
s.split=new Function("l","d",""
+"var i,x=0,a=new Array;while(l){i=l.indexOf(d);i=i>-1?i:l.length;a[x"
+"++]=l.substring(0,i);l=l.substring(i+d.length);}return a");

/*
 * Plugin: getPreviousValue_v1.0 - return previous value of designated
 *   variable (requires split utility)
 */

s.getPreviousValue=new Function("v","c","el",""
+"var s=this,t=new Date,i,j,r='';t.setTime(t.getTime()+1800000);if(el"
+"){if(s.events){i=s.split(el,',');j=s.split(s.events,',');for(x in i"
+"){for(y in j){if(i[x]==j[y]){if(s.c_r(c)) r=s.c_r(c);v?s.c_w(c,v,t)"
+":s.c_w(c,'no value',t);return r}}}}}else{if(s.c_r(c)) r=s.c_r(c);v?"
+"s.c_w(c,v,t):s.c_w(c,'no value',t);return r}");

/*
 * Plugin: getTimeParting 1.3 - Set timeparting values based on time zone
 */

s.getTimeParting=new Function("t","z","y",""
+"dc=new Date('1/1/2000');f=15;ne=8;if(dc.getDay()!=6||"
+"dc.getMonth()!=0){return'Data Not Available'}else{;z=parseInt(z);"
+"if(y=='2009'){f=8;ne=1};gmar=new Date('3/1/'+y);dsts=f-gmar.getDay("
+");gnov=new Date('11/1/'+y);dste=ne-gnov.getDay();spr=new Date('3/'"
+"+dsts+'/'+y);fl=new Date('11/'+dste+'/'+y);cd=new Date();"
+"if(cd>spr&&cd<fl){z=z+1}else{z=z};utc=cd.getTime()+(cd.getTimezoneO"
+"ffset()*60000);tz=new Date(utc + (3600000*z));thisy=tz.getFullYear("
+");var days=['Sunday','Monday','Tuesday','Wednesday','Thursday','Fr"
+"iday','Saturday'];if(thisy!=y){return'Data Not Available'}else{;thi"
+"sh=tz.getHours();thismin=tz.getMinutes();thisd=tz.getDay();var dow="
+"days[thisd];var ap='AM';var dt='Weekday';var mint='00';if(thismin>3"
+"0){mint='30'}if(thish>=12){ap='PM';thish=thish-12};if (thish==0){th"
+"ish=12};if(thisd==6||thisd==0){dt='Weekend'};var timestring=thish+'"
+":'+mint+ap;var daystring=dow;var endstring=dt;if(t=='h'){return tim"
+"estring}if(t=='d'){return daystring};if(t=='w'){return en"
+"dstring}}};"
);

/*
* Plugin: getVisitNum - version 3.0
*/
s.getVisitNum=new Function("tp","c","c2",""
+"var s=this,e=new Date,cval,cvisit,ct=e.getTime(),d;if(!tp){tp='m';}"
+"if(tp=='m'||tp=='w'||tp=='d'){eo=s.endof(tp),y=eo.getTime();e.setTi"
+"me(y);}else {d=tp*86400000;e.setTime(ct+d);}if(!c){c='s_vnum';}if(!"
+"c2){c2='s_invisit';}cval=s.c_r(c);if(cval){var i=cval.indexOf('&vn="
+"'),str=cval.substring(i+4,cval.length),k;}cvisit=s.c_r(c2);if(cvisi"
+"t){if(str){e.setTime(ct+1800000);s.c_w(c2,'true',e);return str;}els"
+"e {return 'unknown visit number';}}else {if(str){str++;k=cval.substri"
+"ng(0,i);e.setTime(k);s.c_w(c,k+'&vn='+str,e);e.setTime(ct+1800000);"
+"s.c_w(c2,'true',e);return str;}else {s.c_w(c,e.getTime()+'&vn=1',e)"
+";e.setTime(ct+1800000);s.c_w(c2,'true',e);return 1;}}");
s.dimo=new Function("m","y",""
+"var d=new Date(y,m+1,0);return d.getDate();");
s.endof=new Function("x",""
+"var t=new Date;t.setHours(0);t.setMinutes(0);t.setSeconds(0);if(x=="
+"'m'){d=s.dimo(t.getMonth(),t.getFullYear())-t.getDate()+1;}else if("
+"x=='w'){d=7-t.getDay();}else {d=1;}t.setDate(t.getDate()+d);return "
+"t;");

/*
* Plugin: getNewRepeat 1.2 - Returns whether user is new or repeat
*/
s.getNewRepeat=new Function("d","cn",""
+"var s=this,e=new Date(),cval,sval,ct=e.getTime();d=d?d:30;cn=cn?cn:"
+"'s_nr';e.setTime(ct+d*24*60*60*1000);cval=s.c_r(cn);if(cval.length="
+"=0){s.c_w(cn,ct+'-New',e);return'New';}sval=s.split(cval,'-');if(ct"
+"-sval[0]<30*60*1000&&sval[1]=='New'){s.c_w(cn,ct+'-New',e);return'N"
+"ew';}else{s.c_w(cn,ct+'-Repeat',e);return'Repeat';}");

s.trackingServer = 'mcclatchy.sc.omtrdc.net';
s.trackingServerSecure = 'mcclatchy.sc.omtrdc.net';
s.loadModule('AudienceManagement');
/*
 Start ActivityMap Module

 The following module enables ActivityMap tracking in Adobe Analytics. ActivityMap
 allows you to view data overlays on your links and content to understand how
 users engage with your web site. If you do not intend to use ActivityMap, you
 can remove the following block of code from your AppMeasurement.js file.
 Additional documentation on how to configure ActivityMap is available at:
 https://marketing.adobe.com/resources/help/en_US/analytics/activitymap/getting-started-admins.html
*/
function AppMeasurement_Module_ActivityMap(f){function g(a,d){var b,c,n;if(a&&d&&(b=e.c[d]||(e.c[d]=d.split(","))))for(n=0;n<b.length&&(c=b[n++]);)if(-1<a.indexOf(c))return null;p=1;return a}function q(a,d,b,c,e){var g,h;if(a.dataset&&(h=a.dataset[d]))g=h;else if(a.getAttribute)if(h=a.getAttribute("data-"+b))g=h;else if(h=a.getAttribute(b))g=h;if(!g&&f.useForcedLinkTracking&&e&&(g="",d=a.onclick?""+a.onclick:"")){b=d.indexOf(c);var l,k;if(0<=b){for(b+=10;b<d.length&&0<="= \t\r\n".indexOf(d.charAt(b));)b++;
if(b<d.length){h=b;for(l=k=0;h<d.length&&(";"!=d.charAt(h)||l);)l?d.charAt(h)!=l||k?k="\\"==d.charAt(h)?!k:0:l=0:(l=d.charAt(h),'"'!=l&&"'"!=l&&(l=0)),h++;if(d=d.substring(b,h))a.e=new Function("s","var e;try{s.w."+c+"="+d+"}catch(e){}"),a.e(f)}}}return g||e&&f.w[c]}function r(a,d,b){var c;return(c=e[d](a,b))&&(p?(p=0,c):g(k(c),e[d+"Exclusions"]))}function s(a,d,b){var c;if(a&&!(1===(c=a.nodeType)&&(c=a.nodeName)&&(c=c.toUpperCase())&&t[c])&&(1===a.nodeType&&(c=a.nodeValue)&&(d[d.length]=c),b.a||
b.t||b.s||!a.getAttribute||((c=a.getAttribute("alt"))?b.a=c:(c=a.getAttribute("title"))?b.t=c:"IMG"==(""+a.nodeName).toUpperCase()&&(c=a.getAttribute("src")||a.src)&&(b.s=c)),(c=a.childNodes)&&c.length))for(a=0;a<c.length;a++)s(c[a],d,b)}function k(a){if(null==a||void 0==a)return a;try{return a.replace(RegExp("^[\\s\\n\\f\\r\\t\t-\r \u00a0\u1680\u180e\u2000-\u200a\u2028\u2029\u205f\u3000\ufeff]+","mg"),"").replace(RegExp("[\\s\\n\\f\\r\\t\t-\r \u00a0\u1680\u180e\u2000-\u200a\u2028\u2029\u205f\u3000\ufeff]+$",
"mg"),"").replace(RegExp("[\\s\\n\\f\\r\\t\t-\r \u00a0\u1680\u180e\u2000-\u200a\u2028\u2029\u205f\u3000\ufeff]{1,}","mg")," ").substring(0,254)}catch(d){}}var e=this;e.s=f;var m=window;m.s_c_in||(m.s_c_il=[],m.s_c_in=0);e._il=m.s_c_il;e._in=m.s_c_in;e._il[e._in]=e;m.s_c_in++;e._c="s_m";e.c={};var p=0,t={SCRIPT:1,STYLE:1,LINK:1,CANVAS:1};e._g=function(){var a,d,b,c=f.contextData,e=f.linkObject;(a=f.pageName||f.pageURL)&&(d=r(e,"link",f.linkName))&&(b=r(e,"region"))&&(c["a.activitymap.page"]=a.substring(0,
255),c["a.activitymap.link"]=128<d.length?d.substring(0,128):d,c["a.activitymap.region"]=127<b.length?b.substring(0,127):b,c["a.activitymap.pageIDType"]=f.pageName?1:0)};e.link=function(a,d){var b;if(d)b=g(k(d),e.linkExclusions);else if((b=a)&&!(b=q(a,"sObjectId","s-object-id","s_objectID",1))){var c,f;(f=g(k(a.innerText||a.textContent),e.linkExclusions))||(s(a,c=[],b={a:void 0,t:void 0,s:void 0}),(f=g(k(c.join(""))))||(f=g(k(b.a?b.a:b.t?b.t:b.s?b.s:void 0)))||!(c=(c=a.tagName)&&c.toUpperCase?c.toUpperCase():
"")||("INPUT"==c||"SUBMIT"==c&&a.value?f=g(k(a.value)):"IMAGE"==c&&a.src&&(f=g(k(a.src)))));b=f}return b};e.region=function(a){for(var d,b=e.regionIDAttribute||"id";a&&(a=a.parentNode);){if(d=q(a,b,b,b))return d;if("BODY"==a.nodeName)return"BODY"}}}
/* End ActivityMap Module */

function AppMeasurement_Module_AudienceManagement(d){var a=this;a.s=d;var b=window;b.s_c_in||(b.s_c_il=[],b.s_c_in=0);a._il=b.s_c_il;a._in=b.s_c_in;a._il[a._in]=a;b.s_c_in++;a._c="s_m";a.setup=function(c){b.DIL&&c&&(c.disableDefaultRequest=!0,c.disableScriptAttachment=!0,c.disableCORS=!0,c.secureDataCollection=!1,a.instance=b.DIL.create(c),a.tools=b.DIL.tools)};a.isReady=function(){return a.instance?!0:!1};a.getEventCallConfigParams=function(){return a.instance&&a.instance.api&&a.instance.api.getEventCallConfigParams?
a.instance.api.getEventCallConfigParams():{}};a.passData=function(b){a.instance&&a.instance.api&&a.instance.api.passData&&a.instance.api.passData(b)}}
"function"!==typeof window.DIL&&(window.DIL=function(c,f){var k=[],g,w;c!==Object(c)&&(c={});var u,l,D,N,A,y,E,F,O,P,G,B,z;u=c.partner;l=c.containerNSID;D=!!c.disableDestinationPublishingIframe;N=c.iframeAkamaiHTTPS;A=c.mappings;y=c.uuidCookie;E=!0===c.enableErrorReporting;F=c.visitorService;O=c.declaredId;P=!0===c.delayAllUntilWindowLoad;G=!0===c.disableIDSyncs;B="undefined"===typeof c.secureDataCollection||!0===c.secureDataCollection;z="boolean"===typeof c.isCoopSafe?c.isCoopSafe:null;var Q,L,H,
R,S;Q=!0===c.disableDefaultRequest;L=c.afterResultForDefaultRequest;H=c.dpIframeSrc;R=c.visitorConstructor;S=!0===c.disableCORS;E&&DIL.errorModule.activate();E=!0===window._dil_unit_tests;(g=f)&&k.push(g+"");if(!u||"string"!==typeof u)return g="DIL partner is invalid or not specified in initConfig",DIL.errorModule.handleError({name:"error",message:g,filename:"dil.js"}),Error(g);g="DIL containerNSID is invalid or not specified in initConfig, setting to default of 0";if(l||"number"===typeof l)l=parseInt(l,
10),!isNaN(l)&&0<=l&&(g="");g&&(l=0,k.push(g),g="");w=DIL.getDil(u,l);if(w instanceof DIL&&w.api.getPartner()===u&&w.api.getContainerNSID()===l)return w;if(this instanceof DIL)DIL.registerDil(this,u,l);else return new DIL(c,"DIL was not instantiated with the 'new' operator, returning a valid instance with partner = "+u+" and containerNSID = "+l);var t={IS_HTTPS:B||"https:"===document.location.protocol,MILLIS_PER_DAY:864E5,DIL_COOKIE_NAME:"AAMC_"+encodeURIComponent(u)+"_"+l,FIRST_PARTY_SYNCS:"AMSYNCS",
FIRST_PARTY_SYNCS_ON_PAGE:"AMSYNCSOP",REGION:"REGION",SIX_MONTHS_IN_MINUTES:259200,IE_VERSION:function(){if(document.documentMode)return document.documentMode;for(var a=7;4<a;a--){var b=document.createElement("div");b.innerHTML="\x3c!--[if IE "+a+"]><span></span><![endif]--\x3e";if(b.getElementsByTagName("span").length)return a}return null}()};t.IS_IE_LESS_THAN_10="number"===typeof t.IE_VERSION&&10>t.IE_VERSION;var M={stuffed:{}},m={},p={firingQueue:[],fired:[],firing:!1,sent:[],errored:[],reservedKeys:{sids:!0,
pdata:!0,logdata:!0,callback:!0,postCallbackFn:!0,useImageRequest:!0},firstRequestHasFired:!1,abortRequests:!1,num_of_cors_responses:0,num_of_cors_errors:0,corsErrorSources:[],num_of_img_responses:0,num_of_img_errors:0,platformParams:{d_nsid:l+"",d_rtbd:"json",d_jsonv:DIL.jsonVersion+"",d_dst:"1"},nonModStatsParams:{d_rtbd:!0,d_dst:!0,d_cts:!0,d_rs:!0},modStatsParams:null,adms:{TIME_TO_CATCH_ALL_REQUESTS_RELEASE:2E3,calledBack:!1,mid:null,noVisitorAPI:!1,VisitorAPI:null,instance:null,releaseType:"no VisitorAPI",
isOptedOut:!0,isOptedOutCallbackCalled:!1,admsProcessingStarted:!1,process:function(a){try{if(!this.admsProcessingStarted){this.admsProcessingStarted=!0;var b=this,e,d,h,n;if("function"===typeof a&&"function"===typeof a.getInstance){if(F===Object(F)&&(e=F.namespace)&&"string"===typeof e)d=a.getInstance(e,{idSyncContainerID:l});else{this.releaseType="no namespace";this.releaseRequests();return}if(d===Object(d)&&d instanceof a&&"function"===typeof d.isAllowed&&"function"===typeof d.getMarketingCloudVisitorID&&
"function"===typeof d.getCustomerIDs&&"function"===typeof d.isOptedOut){this.VisitorAPI=a;if(!d.isAllowed()){this.releaseType="VisitorAPI not allowed";this.releaseRequests();return}this.instance=d;h=function(a){"VisitorAPI"!==b.releaseType&&(b.mid=a,b.releaseType="VisitorAPI",b.releaseRequests())};n=d.getMarketingCloudVisitorID(h);if("string"===typeof n&&n.length){h(n);return}setTimeout(function(){"VisitorAPI"!==b.releaseType&&(b.releaseType="timeout",b.releaseRequests())},this.getLoadTimeout());
return}this.releaseType="invalid instance"}else this.noVisitorAPI=!0;this.releaseRequests()}}catch(c){this.releaseRequests()}},releaseRequests:function(){this.calledBack=!0;p.registerRequest()},getMarketingCloudVisitorID:function(){return this.instance?this.instance.getMarketingCloudVisitorID():null},getMIDQueryString:function(){var a=r.isPopulatedString,b=this.getMarketingCloudVisitorID();a(this.mid)&&this.mid===b||(this.mid=b);return a(this.mid)?"d_mid="+this.mid+"&":""},getCustomerIDs:function(){return this.instance?
this.instance.getCustomerIDs():null},getCustomerIDsQueryString:function(a){if(a===Object(a)){var b="",e=[],d=[],h,n;for(h in a)a.hasOwnProperty(h)&&(d[0]=h,n=a[h],n===Object(n)&&(d[1]=n.id||"",d[2]=n.authState||0,e.push(d),d=[]));if(d=e.length)for(a=0;a<d;a++)b+="&d_cid_ic="+q.encodeAndBuildRequest(e[a],"%01");return b}return""},getIsOptedOut:function(){this.instance?this.instance.isOptedOut([this,this.isOptedOutCallback],this.VisitorAPI.OptOut.GLOBAL,!0):(this.isOptedOut=!1,this.isOptedOutCallbackCalled=
!0)},isOptedOutCallback:function(a){this.isOptedOut=a;this.isOptedOutCallbackCalled=!0;p.registerRequest()},getLoadTimeout:function(){var a=this.instance;if(a){if("function"===typeof a.getLoadTimeout)return a.getLoadTimeout();if("undefined"!==typeof a.loadTimeout)return a.loadTimeout}return this.TIME_TO_CATCH_ALL_REQUESTS_RELEASE}},declaredId:{declaredId:{init:null,request:null},declaredIdCombos:{},setDeclaredId:function(a,b){var e=r.isPopulatedString,d=encodeURIComponent;if(a===Object(a)&&e(b)){var h=
a.dpid,n=a.dpuuid,c=null;if(e(h)&&e(n)){c=d(h)+"$"+d(n);if(!0===this.declaredIdCombos[c])return"setDeclaredId: combo exists for type '"+b+"'";this.declaredIdCombos[c]=!0;this.declaredId[b]={dpid:h,dpuuid:n};return"setDeclaredId: succeeded for type '"+b+"'"}}return"setDeclaredId: failed for type '"+b+"'"},getDeclaredIdQueryString:function(){var a=this.declaredId.request,b=this.declaredId.init,e=encodeURIComponent,d="";null!==a?d="&d_dpid="+e(a.dpid)+"&d_dpuuid="+e(a.dpuuid):null!==b&&(d="&d_dpid="+
e(b.dpid)+"&d_dpuuid="+e(b.dpuuid));return d}},registerRequest:function(a){var b=this.firingQueue;a===Object(a)&&b.push(a);this.firing||!b.length||P&&!DIL.windowLoaded||(this.adms.isOptedOutCallbackCalled||this.adms.getIsOptedOut(),this.adms.calledBack&&!this.adms.isOptedOut&&this.adms.isOptedOutCallbackCalled&&(this.adms.isOptedOutCallbackCalled=!1,a=b.shift(),a.src=a.src.replace(/demdex.net\/event\?d_nsid=/,"demdex.net/event?"+this.adms.getMIDQueryString()+"d_nsid="),r.isPopulatedString(a.corsPostData)&&
(a.corsPostData=a.corsPostData.replace(/^d_nsid=/,this.adms.getMIDQueryString()+"d_nsid=")),C.fireRequest(a),this.firstRequestHasFired||"script"!==a.tag&&"cors"!==a.tag||(this.firstRequestHasFired=!0)))},processVisitorAPI:function(){this.adms.process(R||window.Visitor)},getCoopQueryString:function(){var a="";!0===z?a="&d_coop_safe=1":!1===z&&(a="&d_coop_unsafe=1");return a}};B=function(){var a="http://fast.",b="?d_nsid="+l+"#"+encodeURIComponent(document.location.href);if("string"===typeof H&&H.length)return H+
b;t.IS_HTTPS&&(a=!0===N?"https://fast.":"https://");return a+u+".demdex.net/dest5.html"+b};var v={MAX_SYNCS_LENGTH:649,id:"destination_publishing_iframe_"+u+"_"+l,url:B(),onPagePixels:[],iframeHost:null,getIframeHost:function(a){if("string"===typeof a){var b=a.split("/");if(3<=b.length)return b[0]+"//"+b[2];k.push("getIframeHost: url is malformed: "+a);return a}},iframe:null,iframeHasLoaded:!1,sendingMessages:!1,messages:[],messagesPosted:[],messagesReceived:[],ibsDeleted:[],jsonForComparison:[],
jsonDuplicates:[],jsonWaiting:[],jsonProcessed:[],canSetThirdPartyCookies:!0,receivedThirdPartyCookiesNotification:!1,newIframeCreated:null,iframeIdChanged:!1,originalIframeHasLoadedAlready:null,regionChanged:!1,timesRegionChanged:0,attachIframe:function(){function a(){d=document.createElement("iframe");d.sandbox="allow-scripts allow-same-origin";d.title="Adobe ID Syncing iFrame";d.id=e.id;d.name=e.id+"_name";d.style.cssText="display: none; width: 0; height: 0;";d.src=e.url;e.newIframeCreated=!0;
b();document.body.appendChild(d)}function b(){d.addEventListener("load",function(){d.className="aamIframeLoaded";e.iframeHasLoaded=!0;e.requestToProcess()})}if(!t.IS_IE_LESS_THAN_10){var e=this,d=document.getElementById(this.id);d?"IFRAME"!==d.nodeName?(this.id+="_2",this.iframeIdChanged=!0,a()):(this.newIframeCreated=!1,"aamIframeLoaded"!==d.className?(this.originalIframeHasLoadedAlready=!1,b()):(this.iframeHasLoaded=this.originalIframeHasLoadedAlready=!0,this.iframe=d,this.requestToProcess())):
a();this.iframe=d}},requestToProcess:function(a,b){function e(){d.jsonForComparison.push(a);d.jsonWaiting.push([a,b])}var d=this,h,n;h=p.adms.instance;a===Object(a)&&h===Object(h)&&h.idSyncContainerID===l&&(v.ibsDeleted.push(a.ibs),delete a.ibs);if(a&&!r.isEmptyObject(a))if(h=JSON.stringify(a.ibs||[]),n=JSON.stringify(a.dests||[]),this.jsonForComparison.length){var c=!1,f,g,k;f=0;for(g=this.jsonForComparison.length;f<g;f++)if(k=this.jsonForComparison[f],h===JSON.stringify(k.ibs||[])&&n===JSON.stringify(k.dests||
[])){c=!0;break}c?this.jsonDuplicates.push(a):e()}else e();this.receivedThirdPartyCookiesNotification&&this.jsonWaiting.length&&(h=this.jsonWaiting.shift(),!1===this.newIframeCreated&&delete h[0].ibs,this.process(h[0],h[1]),this.requestToProcess());this.iframeHasLoaded&&this.messages.length&&!this.sendingMessages&&(this.sendingMessages=!0,this.sendMessages())},checkIfRegionChanged:function(a){var b=q.getDilCookieField(t.REGION);null!==b&&"undefined"!==typeof a.dcs_region&&parseInt(b,10)!==a.dcs_region&&
(this.regionChanged=!0,this.timesRegionChanged++,q.setDilCookieField(t.FIRST_PARTY_SYNCS_ON_PAGE,""),q.setDilCookieField(t.FIRST_PARTY_SYNCS,""));"undefined"!==typeof a.dcs_region&&q.setDilCookieField(t.REGION,a.dcs_region)},processSyncOnPage:function(a){var b,e,d;if((b=a.ibs)&&b instanceof Array&&(e=b.length))for(a=0;a<e;a++)d=b[a],d.syncOnPage&&this.checkFirstPartyCookie(d,"","syncOnPage")},process:function(a,b){var e=encodeURIComponent,d,h,n,c,f,g;b===Object(b)&&(g=q.encodeAndBuildRequest(["",
b.dpid||"",b.dpuuid||""],","));if((d=a.dests)&&d instanceof Array&&(h=d.length))for(n=0;n<h;n++)c=d[n],f=[e("dests"),e(c.id||""),e(c.y||""),e(c.c||"")],this.addMessage(f.join("|"));if((d=a.ibs)&&d instanceof Array&&(h=d.length))for(n=0;n<h;n++)c=d[n],f=[e("ibs"),e(c.id||""),e(c.tag||""),q.encodeAndBuildRequest(c.url||[],","),e(c.ttl||""),"",g,c.fireURLSync?"true":"false"],c.syncOnPage||(this.canSetThirdPartyCookies?this.addMessage(f.join("|")):c.fireURLSync&&this.checkFirstPartyCookie(c,f.join("|")));
this.jsonProcessed.push(a)},checkFirstPartyCookie:function(a,b,e){var d=(e="syncOnPage"===e?!0:!1)?t.FIRST_PARTY_SYNCS_ON_PAGE:t.FIRST_PARTY_SYNCS,h=this.getOnPageSyncData(d),c=!1,f=!1,g=Math.ceil((new Date).getTime()/t.MILLIS_PER_DAY);h?(h=h.split("*"),f=this.pruneSyncData(h,a.id,g),c=f.dataPresent,f=f.dataValid,c&&f||this.fireSync(e,a,b,h,d,g)):(h=[],this.fireSync(e,a,b,h,d,g))},getOnPageSyncData:function(a){var b=p.adms.instance;return b&&"function"===typeof b.idSyncGetOnPageSyncInfo?b.idSyncGetOnPageSyncInfo():
q.getDilCookieField(a)},pruneSyncData:function(a,b,e){var d=!1,h=!1,c,f,g;if(a instanceof Array)for(f=0;f<a.length;f++)c=a[f],g=parseInt(c.split("-")[1],10),c.match("^"+b+"-")?(d=!0,e<g?h=!0:(a.splice(f,1),f--)):e>=g&&(a.splice(f,1),f--);return{dataPresent:d,dataValid:h}},manageSyncsSize:function(a){if(a.join("*").length>this.MAX_SYNCS_LENGTH)for(a.sort(function(a,e){return parseInt(a.split("-")[1],10)-parseInt(e.split("-")[1],10)});a.join("*").length>this.MAX_SYNCS_LENGTH;)a.shift()},fireSync:function(a,
b,e,d,h,c){function f(a,b,d,e){return function(){g.onPagePixels[a]=null;var h=g.getOnPageSyncData(d),c=[];if(h){var h=h.split("*"),n,f,k;n=0;for(f=h.length;n<f;n++)k=h[n],k.match("^"+b.id+"-")||c.push(k)}g.setSyncTrackingData(c,b,d,e)}}var g=this;if(a){if("img"===b.tag){a=b.url;e=t.IS_HTTPS?"https:":"http:";var k,p,q;d=0;for(k=a.length;d<k;d++){p=a[d];q=/^\/\//.test(p);var l=new Image;l.addEventListener("load",f(this.onPagePixels.length,b,h,c));l.src=(q?e:"")+p;this.onPagePixels.push(l)}}}else this.addMessage(e),
this.setSyncTrackingData(d,b,h,c)},addMessage:function(a){this.messages.push(a)},setSyncTrackingData:function(a,b,e,d){a.push(b.id+"-"+(d+Math.ceil(b.ttl/60/24)));this.manageSyncsSize(a);q.setDilCookieField(e,a.join("*"))},sendMessages:function(){var a="",b=encodeURIComponent;this.regionChanged&&(a=b("---destpub-clear-dextp---"),this.regionChanged=!1);this.messages.length&&(a=a+b("---destpub-combined---")+this.messages.join("%01"),this.postMessage(a),this.messages=[]);this.sendingMessages=!1},postMessage:function(a){DIL.xd.postMessage(a,
this.url,this.iframe.contentWindow);this.messagesPosted.push(a)},receiveMessage:function(a){var b=/^---destpub-to-parent---/;"string"===typeof a&&b.test(a)&&(b=a.replace(b,"").split("|"),"canSetThirdPartyCookies"===b[0]&&(this.canSetThirdPartyCookies="true"===b[1]?!0:!1,this.receivedThirdPartyCookiesNotification=!0,this.requestToProcess()),this.messagesReceived.push(a))}},J={traits:function(a){r.isValidPdata(a)&&(m.sids instanceof Array||(m.sids=[]),q.extendArray(m.sids,a));return this},pixels:function(a){r.isValidPdata(a)&&
(m.pdata instanceof Array||(m.pdata=[]),q.extendArray(m.pdata,a));return this},logs:function(a){r.isValidLogdata(a)&&(m.logdata!==Object(m.logdata)&&(m.logdata={}),q.extendObject(m.logdata,a));return this},customQueryParams:function(a){r.isEmptyObject(a)||q.extendObject(m,a,p.reservedKeys);return this},signals:function(a,b){var e,d=a;if(!r.isEmptyObject(d)){if(b&&"string"===typeof b)for(e in d={},a)a.hasOwnProperty(e)&&(d[b+e]=a[e]);q.extendObject(m,d,p.reservedKeys)}return this},declaredId:function(a){p.declaredId.setDeclaredId(a,
"request");return this},result:function(a){"function"===typeof a&&(m.callback=a);return this},afterResult:function(a){"function"===typeof a&&(m.postCallbackFn=a);return this},useImageRequest:function(){m.useImageRequest=!0;return this},clearData:function(){m={};return this},submit:function(){C.submitRequest(m);m={};return this},getPartner:function(){return u},getContainerNSID:function(){return l},getEventLog:function(){return k},getState:function(){var a={},b={};q.extendObject(a,p,{registerRequest:!0});
q.extendObject(b,v,{attachIframe:!0,requestToProcess:!0,process:!0,sendMessages:!0});return{initConfig:c,pendingRequest:m,otherRequestInfo:a,destinationPublishingInfo:b}},idSync:function(a){if(G)return"Error: id syncs have been disabled";if(a!==Object(a)||"string"!==typeof a.dpid||!a.dpid.length)return"Error: config or config.dpid is empty";if("string"!==typeof a.url||!a.url.length)return"Error: config.url is empty";var b=a.url,e=a.minutesToLive,d=encodeURIComponent,h=v,c,b=b.replace(/^https:/,"").replace(/^http:/,
"");if("undefined"===typeof e)e=20160;else if(e=parseInt(e,10),isNaN(e)||0>=e)return"Error: config.minutesToLive needs to be a positive number";c=q.encodeAndBuildRequest(["",a.dpid,a.dpuuid||""],",");a=["ibs",d(a.dpid),"img",d(b),e,"",c];h.addMessage(a.join("|"));p.firstRequestHasFired&&h.requestToProcess();return"Successfully queued"},aamIdSync:function(a){if(G)return"Error: id syncs have been disabled";if(a!==Object(a)||"string"!==typeof a.dpuuid||!a.dpuuid.length)return"Error: config or config.dpuuid is empty";
a.url="//dpm.demdex.net/ibs:dpid="+a.dpid+"&dpuuid="+a.dpuuid;return this.idSync(a)},passData:function(a){if(r.isEmptyObject(a))return"Error: json is empty or not an object";v.ibsDeleted.push(a.ibs);delete a.ibs;C.defaultCallback(a);return a},getPlatformParams:function(){return p.platformParams},getEventCallConfigParams:function(){var a=p,b=a.modStatsParams,e=a.platformParams,d;if(!b){b={};for(d in e)e.hasOwnProperty(d)&&!a.nonModStatsParams[d]&&(b[d.replace(/^d_/,"")]=e[d]);!0===z?b.coop_safe=1:
!1===z&&(b.coop_unsafe=1);a.modStatsParams=b}return b},setAsCoopSafe:function(){z=!0;return this},setAsCoopUnsafe:function(){z=!1;return this}},C={corsMetadata:function(){var a="none",b=!0;"undefined"!==typeof XMLHttpRequest&&XMLHttpRequest===Object(XMLHttpRequest)&&("withCredentials"in new XMLHttpRequest?a="XMLHttpRequest":(new Function("/*@cc_on return /^10/.test(@_jscript_version) @*/"))()?a="XMLHttpRequest":"undefined"!==typeof XDomainRequest&&XDomainRequest===Object(XDomainRequest)&&(b=!1),0<
Object.prototype.toString.call(window.HTMLElement).indexOf("Constructor")&&(b=!1));return{corsType:a,corsCookiesEnabled:b}}(),getCORSInstance:function(){return"none"===this.corsMetadata.corsType?null:new window[this.corsMetadata.corsType]},submitRequest:function(a){p.registerRequest(C.createQueuedRequest(a));return!0},createQueuedRequest:function(a){var b=a.callback,e="img",d,h;if(!r.isEmptyObject(A)){var c;for(d in A)A.hasOwnProperty(d)&&(h=A[d],null==h||""===h||!(d in a)||h in a||h in p.reservedKeys||
(c=a[d],null!=c&&""!==c&&(a[h]=c)))}r.isValidPdata(a.sids)||(a.sids=[]);r.isValidPdata(a.pdata)||(a.pdata=[]);r.isValidLogdata(a.logdata)||(a.logdata={});a.logdataArray=q.convertObjectToKeyValuePairs(a.logdata,"=",!0);a.logdataArray.push("_ts="+(new Date).getTime());"function"!==typeof b&&(b=this.defaultCallback);d=this.makeRequestSrcData(a);(h=this.getCORSInstance())&&!0!==a.useImageRequest&&(e="cors");return{tag:e,src:d.src,corsSrc:d.corsSrc,callbackFn:b,postCallbackFn:a.postCallbackFn,useImageRequest:!!a.useImageRequest,
requestData:a,corsInstance:h,corsPostData:d.corsPostData}},defaultCallback:function(a,b){v.checkIfRegionChanged(a);v.processSyncOnPage(a);var e,d,h,c,f,g,k,l,m;if((e=a.stuff)&&e instanceof Array&&(d=e.length))for(h=0;h<d;h++)if((c=e[h])&&c===Object(c)){f=c.cn;g=c.cv;k=c.ttl;if("undefined"===typeof k||""===k)k=Math.floor(q.getMaxCookieExpiresInMinutes()/60/24);l=c.dmn||"."+document.domain.replace(/^www\./,"");m=c.type;f&&(g||"number"===typeof g)&&("var"!==m&&(k=parseInt(k,10))&&!isNaN(k)&&q.setCookie(f,
g,1440*k,"/",l,!1),M.stuffed[f]=g)}e=a.uuid;r.isPopulatedString(e)&&!r.isEmptyObject(y)&&(d=y.path,"string"===typeof d&&d.length||(d="/"),h=parseInt(y.days,10),isNaN(h)&&(h=100),q.setCookie(y.name||"aam_did",e,1440*h,d,y.domain||"."+document.domain.replace(/^www\./,""),!0===y.secure));D||p.abortRequests||v.requestToProcess(a,b)},makeRequestSrcData:function(a){a.sids=r.removeEmptyArrayValues(a.sids||[]);a.pdata=r.removeEmptyArrayValues(a.pdata||[]);var b=p,e=b.platformParams,d=q.encodeAndBuildRequest(a.sids,
","),c=q.encodeAndBuildRequest(a.pdata,","),f=(a.logdataArray||[]).join("&");delete a.logdataArray;var g=t.IS_HTTPS?"https://":"http://",k=b.declaredId.getDeclaredIdQueryString(),l=b.adms.instance?b.adms.getCustomerIDsQueryString(b.adms.getCustomerIDs()):"",m=[],x,v,I,w;for(x in a)if(!(x in b.reservedKeys)&&a.hasOwnProperty(x))if(v=a[x],x=encodeURIComponent(x),v instanceof Array)for(I=0,w=v.length;I<w;I++)m.push(x+"="+encodeURIComponent(v[I]));else m.push(x+"="+encodeURIComponent(v));a=m.length?"&"+
m.join("&"):"";b="d_nsid="+e.d_nsid+b.getCoopQueryString()+k+l+(d.length?"&d_sid="+d:"")+(c.length?"&d_px="+c:"")+(f.length?"&d_ld="+encodeURIComponent(f):"");e="&d_rtbd="+e.d_rtbd+"&d_jsonv="+e.d_jsonv+"&d_dst="+e.d_dst;g=g+u+".demdex.net/event";c=d=g+"?"+b+e+a;2048<d.length&&(d=d.substring(0,2048).substring(0,d.lastIndexOf("&")));return{corsSrc:g+"?_ts="+(new Date).getTime(),src:d,originalSrc:c,corsPostData:b+e+a,isDeclaredIdCall:""!==k}},fireRequest:function(a){if("img"===a.tag)this.fireImage(a);
else{var b=p.declaredId,b=b.declaredId.request||b.declaredId.init||{};this.fireCORS(a,{dpid:b.dpid||"",dpuuid:b.dpuuid||""})}},fireImage:function(a){var b=p,e,d;b.abortRequests||(b.firing=!0,e=new Image(0,0),b.sent.push(a),e.onload=function(){b.firing=!1;b.fired.push(a);b.num_of_img_responses++;b.registerRequest()},d=function(d){g="imgAbortOrErrorHandler received the event of type "+d.type;k.push(g);b.abortRequests=!0;b.firing=!1;b.errored.push(a);b.num_of_img_errors++;b.registerRequest()},e.addEventListener("error",
d),e.addEventListener("abort",d),e.src=a.src)},fireCORS:function(a,b){var e=this,d=p,c=this.corsMetadata.corsType,f=a.corsSrc,l=a.corsInstance,q=a.corsPostData,m=a.postCallbackFn,r="function"===typeof m;if(!d.abortRequests&&!S){d.firing=!0;try{l.open("post",f,!0),"XMLHttpRequest"===c&&(l.withCredentials=!0,l.setRequestHeader("Content-Type","application/x-www-form-urlencoded"),l.onreadystatechange=function(){if(4===this.readyState&&200===this.status)a:{var c;try{if(c=JSON.parse(this.responseText),
c!==Object(c)){e.handleCORSError(a,b,"Response is not JSON");break a}}catch(h){e.handleCORSError(a,b,"Error parsing response as JSON");break a}G&&(v.ibsDeleted.push(c.ibs),delete c.ibs);try{var f=a.callbackFn;d.firing=!1;d.fired.push(a);d.num_of_cors_responses++;f(c,b);r&&m(c,b)}catch(h){h.message="DIL handleCORSResponse caught error with message "+h.message;g=h.message;k.push(g);h.filename=h.filename||"dil.js";h.partner=u;DIL.errorModule.handleError(h);try{f({error:h.name+"|"+h.message},b),r&&m({error:h.name+
"|"+h.message},b)}catch(n){}}finally{d.registerRequest()}}}),l.onerror=function(){e.handleCORSError(a,b,"onerror")},l.ontimeout=function(){e.handleCORSError(a,b,"ontimeout")},l.send(q)}catch(t){this.handleCORSError(a,b,"try-catch")}d.sent.push(a);d.declaredId.declaredId.request=null}},handleCORSError:function(a,b,e){p.num_of_cors_errors++;p.corsErrorSources.push(e)},handleRequestError:function(a,b){var e=p;k.push(a);e.abortRequests=!0;e.firing=!1;e.errored.push(b);e.registerRequest()}},r={isValidPdata:function(a){return a instanceof
Array&&this.removeEmptyArrayValues(a).length?!0:!1},isValidLogdata:function(a){return!this.isEmptyObject(a)},isEmptyObject:function(a){if(a!==Object(a))return!0;for(var b in a)if(a.hasOwnProperty(b))return!1;return!0},removeEmptyArrayValues:function(a){for(var b=0,e=a.length,d,c=[],b=0;b<e;b++)d=a[b],"undefined"!==typeof d&&null!==d&&""!==d&&c.push(d);return c},isPopulatedString:function(a){return"string"===typeof a&&a.length}},q={convertObjectToKeyValuePairs:function(a,b,e){var d=[],c,f;b||(b="=");
for(c in a)a.hasOwnProperty(c)&&(f=a[c],"undefined"!==typeof f&&null!==f&&""!==f&&d.push(c+b+(e?encodeURIComponent(f):f)));return d},encodeAndBuildRequest:function(a,b){return a.map(function(a){return encodeURIComponent(a)}).join(b)},getCookie:function(a){a+="=";var b=document.cookie.split(";"),e,d,c;e=0;for(d=b.length;e<d;e++){for(c=b[e];" "===c.charAt(0);)c=c.substring(1,c.length);if(0===c.indexOf(a))return decodeURIComponent(c.substring(a.length,c.length))}return null},setCookie:function(a,b,e,
d,c,f){var g=new Date;e&&(e*=6E4);document.cookie=a+"="+encodeURIComponent(b)+(e?";expires="+(new Date(g.getTime()+e)).toUTCString():"")+(d?";path="+d:"")+(c?";domain="+c:"")+(f?";secure":"")},extendArray:function(a,b){return a instanceof Array&&b instanceof Array?(Array.prototype.push.apply(a,b),!0):!1},extendObject:function(a,b,e){var d;if(a===Object(a)&&b===Object(b)){for(d in b)!b.hasOwnProperty(d)||!r.isEmptyObject(e)&&d in e||(a[d]=b[d]);return!0}return!1},getMaxCookieExpiresInMinutes:function(){return t.SIX_MONTHS_IN_MINUTES},
getCookieField:function(a,b){var e=this.getCookie(a),d=decodeURIComponent;if("string"===typeof e){var e=e.split("|"),c,f;c=0;for(f=e.length-1;c<f;c++)if(d(e[c])===b)return d(e[c+1])}return null},getDilCookieField:function(a){return this.getCookieField(t.DIL_COOKIE_NAME,a)},setCookieField:function(a,b,c){var d=this.getCookie(a),f=!1,g=encodeURIComponent;b=g(b);c=g(c);if("string"===typeof d){var d=d.split("|"),k,g=0;for(k=d.length-1;g<k;g++)if(d[g]===b){d[g+1]=c;f=!0;break}f||(g=d.length,d[g]=b,d[g+
1]=c)}else d=[b,c];this.setCookie(a,d.join("|"),this.getMaxCookieExpiresInMinutes(),"/",this.getDomain(),!1)},setDilCookieField:function(a,b){return this.setCookieField(t.DIL_COOKIE_NAME,a,b)},getDomain:function(a){!a&&window.location&&(a=window.location.hostname);if(a)if(/^[0-9.]+$/.test(a))a="";else{var b=a.split("."),c=b.length-1,d=c-1;1<c&&2>=b[c].length&&(2===b[c-1].length||0>",DOMAIN_2_CHAR_EXCEPTIONS,".indexOf(","+b[c]+","))&&d--;if(0<d)for(a="";c>=d;)a=b[c]+(a?".":"")+a,c--}return a},replaceMethodsWithFunction:function(a,
b){var c;if(a===Object(a)&&"function"===typeof b)for(c in a)a.hasOwnProperty(c)&&"function"===typeof a[c]&&(a[c]=b)}};"error"===u&&0===l&&window.addEventListener("load",function(){DIL.windowLoaded=!0});var T=!1,K=function(){T||(T=!0,p.registerRequest(),U(),D||p.abortRequests||v.attachIframe())},U=function(){D||setTimeout(function(){Q||p.firstRequestHasFired||("function"===typeof L?J.afterResult(L).submit():J.submit())},DIL.constants.TIME_TO_DEFAULT_REQUEST)};w=document;"error"!==u&&(DIL.windowLoaded?
K():"complete"!==w.readyState&&"loaded"!==w.readyState?window.addEventListener("load",function(){DIL.windowLoaded=!0;K()}):(DIL.windowLoaded=!0,K()));if("error"!==u)try{DIL.xd.receiveMessage(function(a){v.receiveMessage(a.data)},v.getIframeHost(v.url))}catch(a){}p.declaredId.setDeclaredId(O,"init");p.processVisitorAPI();t.IS_IE_LESS_THAN_10&&q.replaceMethodsWithFunction(J,function(){return this});this.api=J;this.getStuffedVariable=function(a){var b=M.stuffed[a];b||"number"===typeof b||(b=q.getCookie(a))||
"number"===typeof b||(b="");return b};this.validators=r;this.helpers=q;this.constants=t;this.log=k;E&&(this.pendingRequest=m,this.requestController=p,this.setDestinationPublishingUrl=B,this.destinationPublishing=v,this.requestProcs=C,this.variables=M,this.callWindowLoadFunctions=K)},DIL.extendStaticPropertiesAndMethods=function(c){var f;if(c===Object(c))for(f in c)c.hasOwnProperty(f)&&(this[f]=c[f])},DIL.extendStaticPropertiesAndMethods({version:"7.0",jsonVersion:1,constants:{TIME_TO_DEFAULT_REQUEST:50},
variables:{scriptNodeList:document.getElementsByTagName("script")},windowLoaded:!1,dils:{},isAddedPostWindowLoad:function(c){this.windowLoaded="function"===typeof c?!!c():"boolean"===typeof c?c:!0},create:function(c){try{return new DIL(c)}catch(f){throw Error("Error in attempt to create DIL instance with DIL.create(): "+f.message);}},registerDil:function(c,f,k){f=f+"$"+k;f in this.dils||(this.dils[f]=c)},getDil:function(c,f){var k;"string"!==typeof c&&(c="");f||(f=0);k=c+"$"+f;return k in this.dils?
this.dils[k]:Error("The DIL instance with partner = "+c+" and containerNSID = "+f+" was not found")},dexGetQSVars:function(c,f,k){f=this.getDil(f,k);return f instanceof this?f.getStuffedVariable(c):""},xd:{postMessage:function(c,f,k){f&&k.postMessage(c,f.replace(/([^:]+:\/\/[^\/]+).*/,"$1"))},receiveMessage:function(c,f){var k;try{c&&(k=function(g){if("string"===typeof f&&g.origin!==f||"[object Function]"===Object.prototype.toString.call(f)&&!1===f(g.origin))return!1;c(g)}),window[c?"addEventListener":
"removeEventListener"]("message",k,!1)}catch(g){}}}}),DIL.errorModule=function(){var c=DIL.create({partner:"error",containerNSID:0,disableDestinationPublishingIframe:!0}),f={harvestererror:14138,destpuberror:14139,dpmerror:14140,generalerror:14137,error:14137,noerrortypedefined:15021,evalerror:15016,rangeerror:15017,referenceerror:15018,typeerror:15019,urierror:15020},k=!1;return{activate:function(){k=!0},handleError:function(g){if(!k)return"DIL error module has not been activated";g!==Object(g)&&
(g={});var w=g.name?(g.name+"").toLowerCase():"",u=[];g={name:w,filename:g.filename?g.filename+"":"",partner:g.partner?g.partner+"":"no_partner",site:g.site?g.site+"":document.location.href,message:g.message?g.message+"":""};u.push(w in f?f[w]:f.noerrortypedefined);c.api.pixels(u).logs(g).useImageRequest().submit();return"DIL error report sent"},pixelMap:f}}(),DIL.tools={},DIL.modules={helpers:{handleModuleError:function(c,f,k){var g="";f=f||"Error caught in DIL module/submodule: ";c===Object(c)?
g=f+(c.message||"err has no message"):(g=f+"err is not a valid object",c={});c.message=g;k instanceof DIL&&(c.partner=k.api.getPartner());DIL.errorModule.handleError(c);return this.errorMessage=g}}});

/*
 ============== DO NOT ALTER ANYTHING BELOW THIS LINE ! ===============

AppMeasurement for JavaScript version: 2.8.1
Copyright 1996-2016 Adobe, Inc. All Rights Reserved
More info available at http://www.adobe.com/marketing-cloud.html
*/
function AppMeasurement(r){var a=this;a.version="2.8.1";var k=window;k.s_c_in||(k.s_c_il=[],k.s_c_in=0);a._il=k.s_c_il;a._in=k.s_c_in;a._il[a._in]=a;k.s_c_in++;a._c="s_c";var p=k.AppMeasurement.Xb;p||(p=null);var n=k,m,s;try{for(m=n.parent,s=n.location;m&&m.location&&s&&""+m.location!=""+s&&n.location&&""+m.location!=""+n.location&&m.location.host==s.host;)n=m,m=n.parent}catch(u){}a.F=function(a){try{console.log(a)}catch(b){}};a.Oa=function(a){return""+parseInt(a)==""+a};a.replace=function(a,b,d){return!a||
0>a.indexOf(b)?a:a.split(b).join(d)};a.escape=function(c){var b,d;if(!c)return c;c=encodeURIComponent(c);for(b=0;7>b;b++)d="+~!*()'".substring(b,b+1),0<=c.indexOf(d)&&(c=a.replace(c,d,"%"+d.charCodeAt(0).toString(16).toUpperCase()));return c};a.unescape=function(c){if(!c)return c;c=0<=c.indexOf("+")?a.replace(c,"+"," "):c;try{return decodeURIComponent(c)}catch(b){}return unescape(c)};a.Fb=function(){var c=k.location.hostname,b=a.fpCookieDomainPeriods,d;b||(b=a.cookieDomainPeriods);if(c&&!a.Ga&&!/^[0-9.]+$/.test(c)&&
(b=b?parseInt(b):2,b=2<b?b:2,d=c.lastIndexOf("."),0<=d)){for(;0<=d&&1<b;)d=c.lastIndexOf(".",d-1),b--;a.Ga=0<d?c.substring(d):c}return a.Ga};a.c_r=a.cookieRead=function(c){c=a.escape(c);var b=" "+a.d.cookie,d=b.indexOf(" "+c+"="),f=0>d?d:b.indexOf(";",d);c=0>d?"":a.unescape(b.substring(d+2+c.length,0>f?b.length:f));return"[[B]]"!=c?c:""};a.c_w=a.cookieWrite=function(c,b,d){var f=a.Fb(),e=a.cookieLifetime,g;b=""+b;e=e?(""+e).toUpperCase():"";d&&"SESSION"!=e&&"NONE"!=e&&((g=""!=b?parseInt(e?e:0):-60)?
(d=new Date,d.setTime(d.getTime()+1E3*g)):1==d&&(d=new Date,g=d.getYear(),d.setYear(g+5+(1900>g?1900:0))));return c&&"NONE"!=e?(a.d.cookie=a.escape(c)+"="+a.escape(""!=b?b:"[[B]]")+"; path=/;"+(d&&"SESSION"!=e?" expires="+d.toUTCString()+";":"")+(f?" domain="+f+";":""),a.cookieRead(c)==b):0};a.Cb=function(){var c=a.Util.getIeVersion();"number"===typeof c&&10>c&&(a.unsupportedBrowser=!0,a.rb(a,function(){}))};a.rb=function(a,b){for(var d in a)a.hasOwnProperty(d)&&"function"===typeof a[d]&&(a[d]=b)};
a.L=[];a.ja=function(c,b,d){if(a.Ha)return 0;a.maxDelay||(a.maxDelay=250);var f=0,e=(new Date).getTime()+a.maxDelay,g=a.d.visibilityState,h=["webkitvisibilitychange","visibilitychange"];g||(g=a.d.webkitVisibilityState);if(g&&"prerender"==g){if(!a.ka)for(a.ka=1,d=0;d<h.length;d++)a.d.addEventListener(h[d],function(){var c=a.d.visibilityState;c||(c=a.d.webkitVisibilityState);"visible"==c&&(a.ka=0,a.delayReady())});f=1;e=0}else d||a.p("_d")&&(f=1);f&&(a.L.push({m:c,a:b,t:e}),a.ka||setTimeout(a.delayReady,
a.maxDelay));return f};a.delayReady=function(){var c=(new Date).getTime(),b=0,d;for(a.p("_d")?b=1:a.za();0<a.L.length;){d=a.L.shift();if(b&&!d.t&&d.t>c){a.L.unshift(d);setTimeout(a.delayReady,parseInt(a.maxDelay/2));break}a.Ha=1;a[d.m].apply(a,d.a);a.Ha=0}};a.setAccount=a.sa=function(c){var b,d;if(!a.ja("setAccount",arguments))if(a.account=c,a.allAccounts)for(b=a.allAccounts.concat(c.split(",")),a.allAccounts=[],b.sort(),d=0;d<b.length;d++)0!=d&&b[d-1]==b[d]||a.allAccounts.push(b[d]);else a.allAccounts=
c.split(",")};a.foreachVar=function(c,b){var d,f,e,g,h="";e=f="";if(a.lightProfileID)d=a.P,(h=a.lightTrackVars)&&(h=","+h+","+a.oa.join(",")+",");else{d=a.g;if(a.pe||a.linkType)h=a.linkTrackVars,f=a.linkTrackEvents,a.pe&&(e=a.pe.substring(0,1).toUpperCase()+a.pe.substring(1),a[e]&&(h=a[e].Vb,f=a[e].Ub));h&&(h=","+h+","+a.H.join(",")+",");f&&h&&(h+=",events,")}b&&(b=","+b+",");for(f=0;f<d.length;f++)e=d[f],(g=a[e])&&(!h||0<=h.indexOf(","+e+","))&&(!b||0<=b.indexOf(","+e+","))&&c(e,g)};a.r=function(c,
b,d,f,e){var g="",h,l,k,q,m=0;"contextData"==c&&(c="c");if(b){for(h in b)if(!(Object.prototype[h]||e&&h.substring(0,e.length)!=e)&&b[h]&&(!d||0<=d.indexOf(","+(f?f+".":"")+h+","))){k=!1;if(m)for(l=0;l<m.length;l++)h.substring(0,m[l].length)==m[l]&&(k=!0);if(!k&&(""==g&&(g+="&"+c+"."),l=b[h],e&&(h=h.substring(e.length)),0<h.length))if(k=h.indexOf("."),0<k)l=h.substring(0,k),k=(e?e:"")+l+".",m||(m=[]),m.push(k),g+=a.r(l,b,d,f,k);else if("boolean"==typeof l&&(l=l?"true":"false"),l){if("retrieveLightData"==
f&&0>e.indexOf(".contextData."))switch(k=h.substring(0,4),q=h.substring(4),h){case "transactionID":h="xact";break;case "channel":h="ch";break;case "campaign":h="v0";break;default:a.Oa(q)&&("prop"==k?h="c"+q:"eVar"==k?h="v"+q:"list"==k?h="l"+q:"hier"==k&&(h="h"+q,l=l.substring(0,255)))}g+="&"+a.escape(h)+"="+a.escape(l)}}""!=g&&(g+="&."+c)}return g};a.usePostbacks=0;a.Ib=function(){var c="",b,d,f,e,g,h,l,k,q="",m="",n=e="";if(a.lightProfileID)b=a.P,(q=a.lightTrackVars)&&(q=","+q+","+a.oa.join(",")+
",");else{b=a.g;if(a.pe||a.linkType)q=a.linkTrackVars,m=a.linkTrackEvents,a.pe&&(e=a.pe.substring(0,1).toUpperCase()+a.pe.substring(1),a[e]&&(q=a[e].Vb,m=a[e].Ub));q&&(q=","+q+","+a.H.join(",")+",");m&&(m=","+m+",",q&&(q+=",events,"));a.events2&&(n+=(""!=n?",":"")+a.events2)}if(a.visitor&&a.visitor.getCustomerIDs){e=p;if(g=a.visitor.getCustomerIDs())for(d in g)Object.prototype[d]||(f=g[d],"object"==typeof f&&(e||(e={}),f.id&&(e[d+".id"]=f.id),f.authState&&(e[d+".as"]=f.authState)));e&&(c+=a.r("cid",
e))}a.AudienceManagement&&a.AudienceManagement.isReady()&&(c+=a.r("d",a.AudienceManagement.getEventCallConfigParams()));for(d=0;d<b.length;d++){e=b[d];g=a[e];f=e.substring(0,4);h=e.substring(4);g||("events"==e&&n?(g=n,n=""):"marketingCloudOrgID"==e&&a.visitor&&(g=a.visitor.marketingCloudOrgID));if(g&&(!q||0<=q.indexOf(","+e+","))){switch(e){case "customerPerspective":e="cp";break;case "marketingCloudOrgID":e="mcorgid";break;case "supplementalDataID":e="sdid";break;case "timestamp":e="ts";break;case "dynamicVariablePrefix":e=
"D";break;case "visitorID":e="vid";break;case "marketingCloudVisitorID":e="mid";break;case "analyticsVisitorID":e="aid";break;case "audienceManagerLocationHint":e="aamlh";break;case "audienceManagerBlob":e="aamb";break;case "authState":e="as";break;case "pageURL":e="g";255<g.length&&(a.pageURLRest=g.substring(255),g=g.substring(0,255));break;case "pageURLRest":e="-g";break;case "referrer":e="r";break;case "vmk":case "visitorMigrationKey":e="vmt";break;case "visitorMigrationServer":e="vmf";a.ssl&&
a.visitorMigrationServerSecure&&(g="");break;case "visitorMigrationServerSecure":e="vmf";!a.ssl&&a.visitorMigrationServer&&(g="");break;case "charSet":e="ce";break;case "visitorNamespace":e="ns";break;case "cookieDomainPeriods":e="cdp";break;case "cookieLifetime":e="cl";break;case "variableProvider":e="vvp";break;case "currencyCode":e="cc";break;case "channel":e="ch";break;case "transactionID":e="xact";break;case "campaign":e="v0";break;case "latitude":e="lat";break;case "longitude":e="lon";break;
case "resolution":e="s";break;case "colorDepth":e="c";break;case "javascriptVersion":e="j";break;case "javaEnabled":e="v";break;case "cookiesEnabled":e="k";break;case "browserWidth":e="bw";break;case "browserHeight":e="bh";break;case "connectionType":e="ct";break;case "homepage":e="hp";break;case "events":n&&(g+=(""!=g?",":"")+n);if(m)for(h=g.split(","),g="",f=0;f<h.length;f++)l=h[f],k=l.indexOf("="),0<=k&&(l=l.substring(0,k)),k=l.indexOf(":"),0<=k&&(l=l.substring(0,k)),0<=m.indexOf(","+l+",")&&(g+=
(g?",":"")+h[f]);break;case "events2":g="";break;case "contextData":c+=a.r("c",a[e],q,e);g="";break;case "lightProfileID":e="mtp";break;case "lightStoreForSeconds":e="mtss";a.lightProfileID||(g="");break;case "lightIncrementBy":e="mti";a.lightProfileID||(g="");break;case "retrieveLightProfiles":e="mtsr";break;case "deleteLightProfiles":e="mtsd";break;case "retrieveLightData":a.retrieveLightProfiles&&(c+=a.r("mts",a[e],q,e));g="";break;default:a.Oa(h)&&("prop"==f?e="c"+h:"eVar"==f?e="v"+h:"list"==
f?e="l"+h:"hier"==f&&(e="h"+h,g=g.substring(0,255)))}g&&(c+="&"+e+"="+("pev"!=e.substring(0,3)?a.escape(g):g))}"pev3"==e&&a.e&&(c+=a.e)}a.na&&(c+="&lrt="+a.na,a.na=null);return c};a.D=function(a){var b=a.tagName;if("undefined"!=""+a.$b||"undefined"!=""+a.Qb&&"HTML"!=(""+a.Qb).toUpperCase())return"";b=b&&b.toUpperCase?b.toUpperCase():"";"SHAPE"==b&&(b="");b&&(("INPUT"==b||"BUTTON"==b)&&a.type&&a.type.toUpperCase?b=a.type.toUpperCase():!b&&a.href&&(b="A"));return b};a.Ka=function(a){var b=k.location,
d=a.href?a.href:"",f,e,g;f=d.indexOf(":");e=d.indexOf("?");g=d.indexOf("/");d&&(0>f||0<=e&&f>e||0<=g&&f>g)&&(e=a.protocol&&1<a.protocol.length?a.protocol:b.protocol?b.protocol:"",f=b.pathname.lastIndexOf("/"),d=(e?e+"//":"")+(a.host?a.host:b.host?b.host:"")+("/"!=d.substring(0,1)?b.pathname.substring(0,0>f?0:f)+"/":"")+d);return d};a.M=function(c){var b=a.D(c),d,f,e="",g=0;return b&&(d=c.protocol,f=c.onclick,!c.href||"A"!=b&&"AREA"!=b||f&&d&&!(0>d.toLowerCase().indexOf("javascript"))?f?(e=a.replace(a.replace(a.replace(a.replace(""+
f,"\r",""),"\n",""),"\t","")," ",""),g=2):"INPUT"==b||"SUBMIT"==b?(c.value?e=c.value:c.innerText?e=c.innerText:c.textContent&&(e=c.textContent),g=3):"IMAGE"==b&&c.src&&(e=c.src):e=a.Ka(c),e)?{id:e.substring(0,100),type:g}:0};a.Yb=function(c){for(var b=a.D(c),d=a.M(c);c&&!d&&"BODY"!=b;)if(c=c.parentElement?c.parentElement:c.parentNode)b=a.D(c),d=a.M(c);d&&"BODY"!=b||(c=0);c&&(b=c.onclick?""+c.onclick:"",0<=b.indexOf(".tl(")||0<=b.indexOf(".trackLink("))&&(c=0);return c};a.Pb=function(){var c,b,d=a.linkObject,
f=a.linkType,e=a.linkURL,g,h;a.pa=1;d||(a.pa=0,d=a.clickObject);if(d){c=a.D(d);for(b=a.M(d);d&&!b&&"BODY"!=c;)if(d=d.parentElement?d.parentElement:d.parentNode)c=a.D(d),b=a.M(d);b&&"BODY"!=c||(d=0);if(d&&!a.linkObject){var l=d.onclick?""+d.onclick:"";if(0<=l.indexOf(".tl(")||0<=l.indexOf(".trackLink("))d=0}}else a.pa=1;!e&&d&&(e=a.Ka(d));e&&!a.linkLeaveQueryString&&(g=e.indexOf("?"),0<=g&&(e=e.substring(0,g)));if(!f&&e){var m=0,q=0,n;if(a.trackDownloadLinks&&a.linkDownloadFileTypes)for(l=e.toLowerCase(),
g=l.indexOf("?"),h=l.indexOf("#"),0<=g?0<=h&&h<g&&(g=h):g=h,0<=g&&(l=l.substring(0,g)),g=a.linkDownloadFileTypes.toLowerCase().split(","),h=0;h<g.length;h++)(n=g[h])&&l.substring(l.length-(n.length+1))=="."+n&&(f="d");if(a.trackExternalLinks&&!f&&(l=e.toLowerCase(),a.Na(l)&&(a.linkInternalFilters||(a.linkInternalFilters=k.location.hostname),g=0,a.linkExternalFilters?(g=a.linkExternalFilters.toLowerCase().split(","),m=1):a.linkInternalFilters&&(g=a.linkInternalFilters.toLowerCase().split(",")),g))){for(h=
0;h<g.length;h++)n=g[h],0<=l.indexOf(n)&&(q=1);q?m&&(f="e"):m||(f="e")}}a.linkObject=d;a.linkURL=e;a.linkType=f;if(a.trackClickMap||a.trackInlineStats)a.e="",d&&(f=a.pageName,e=1,d=d.sourceIndex,f||(f=a.pageURL,e=0),k.s_objectID&&(b.id=k.s_objectID,d=b.type=1),f&&b&&b.id&&c&&(a.e="&pid="+a.escape(f.substring(0,255))+(e?"&pidt="+e:"")+"&oid="+a.escape(b.id.substring(0,100))+(b.type?"&oidt="+b.type:"")+"&ot="+c+(d?"&oi="+d:"")))};a.Jb=function(){var c=a.pa,b=a.linkType,d=a.linkURL,f=a.linkName;b&&(d||
f)&&(b=b.toLowerCase(),"d"!=b&&"e"!=b&&(b="o"),a.pe="lnk_"+b,a.pev1=d?a.escape(d):"",a.pev2=f?a.escape(f):"",c=1);a.abort&&(c=0);if(a.trackClickMap||a.trackInlineStats||a.ActivityMap){var b={},d=0,e=a.cookieRead("s_sq"),g=e?e.split("&"):0,h,l,k,e=0;if(g)for(h=0;h<g.length;h++)l=g[h].split("="),f=a.unescape(l[0]).split(","),l=a.unescape(l[1]),b[l]=f;f=a.account.split(",");h={};for(k in a.contextData)k&&!Object.prototype[k]&&"a.activitymap."==k.substring(0,14)&&(h[k]=a.contextData[k],a.contextData[k]=
"");a.e=a.r("c",h)+(a.e?a.e:"");if(c||a.e){c&&!a.e&&(e=1);for(l in b)if(!Object.prototype[l])for(k=0;k<f.length;k++)for(e&&(g=b[l].join(","),g==a.account&&(a.e+=("&"!=l.charAt(0)?"&":"")+l,b[l]=[],d=1)),h=0;h<b[l].length;h++)g=b[l][h],g==f[k]&&(e&&(a.e+="&u="+a.escape(g)+("&"!=l.charAt(0)?"&":"")+l+"&u=0"),b[l].splice(h,1),d=1);c||(d=1);if(d){e="";h=2;!c&&a.e&&(e=a.escape(f.join(","))+"="+a.escape(a.e),h=1);for(l in b)!Object.prototype[l]&&0<h&&0<b[l].length&&(e+=(e?"&":"")+a.escape(b[l].join(","))+
"="+a.escape(l),h--);a.cookieWrite("s_sq",e)}}}return c};a.Kb=function(){if(!a.Tb){var c=new Date,b=n.location,d,f,e=f=d="",g="",h="",l="1.2",k=a.cookieWrite("s_cc","true",0)?"Y":"N",m="",p="";if(c.setUTCDate&&(l="1.3",(0).toPrecision&&(l="1.5",c=[],c.forEach))){l="1.6";f=0;d={};try{f=new Iterator(d),f.next&&(l="1.7",c.reduce&&(l="1.8",l.trim&&(l="1.8.1",Date.parse&&(l="1.8.2",Object.create&&(l="1.8.5")))))}catch(r){}}d=screen.width+"x"+screen.height;e=navigator.javaEnabled()?"Y":"N";f=screen.pixelDepth?
screen.pixelDepth:screen.colorDepth;g=a.w.innerWidth?a.w.innerWidth:a.d.documentElement.offsetWidth;h=a.w.innerHeight?a.w.innerHeight:a.d.documentElement.offsetHeight;try{a.b.addBehavior("#default#homePage"),m=a.b.Zb(b)?"Y":"N"}catch(s){}try{a.b.addBehavior("#default#clientCaps"),p=a.b.connectionType}catch(t){}a.resolution=d;a.colorDepth=f;a.javascriptVersion=l;a.javaEnabled=e;a.cookiesEnabled=k;a.browserWidth=g;a.browserHeight=h;a.connectionType=p;a.homepage=m;a.Tb=1}};a.Q={};a.loadModule=function(c,
b){var d=a.Q[c];if(!d){d=k["AppMeasurement_Module_"+c]?new k["AppMeasurement_Module_"+c](a):{};a.Q[c]=a[c]=d;d.kb=function(){return d.qb};d.sb=function(b){if(d.qb=b)a[c+"_onLoad"]=b,a.ja(c+"_onLoad",[a,d],1)||b(a,d)};try{Object.defineProperty?Object.defineProperty(d,"onLoad",{get:d.kb,set:d.sb}):d._olc=1}catch(f){d._olc=1}}b&&(a[c+"_onLoad"]=b,a.ja(c+"_onLoad",[a,d],1)||b(a,d))};a.p=function(c){var b,d;for(b in a.Q)if(!Object.prototype[b]&&(d=a.Q[b])&&(d._olc&&d.onLoad&&(d._olc=0,d.onLoad(a,d)),d[c]&&
d[c]()))return 1;return 0};a.Mb=function(){var c=Math.floor(1E13*Math.random()),b=a.visitorSampling,d=a.visitorSamplingGroup,d="s_vsn_"+(a.visitorNamespace?a.visitorNamespace:a.account)+(d?"_"+d:""),f=a.cookieRead(d);if(b){b*=100;f&&(f=parseInt(f));if(!f){if(!a.cookieWrite(d,c))return 0;f=c}if(f%1E4>b)return 0}return 1};a.R=function(c,b){var d,f,e,g,h,l;for(d=0;2>d;d++)for(f=0<d?a.Ca:a.g,e=0;e<f.length;e++)if(g=f[e],(h=c[g])||c["!"+g]){if(!b&&("contextData"==g||"retrieveLightData"==g)&&a[g])for(l in a[g])h[l]||
(h[l]=a[g][l]);a[g]=h}};a.Ya=function(c,b){var d,f,e,g;for(d=0;2>d;d++)for(f=0<d?a.Ca:a.g,e=0;e<f.length;e++)g=f[e],c[g]=a[g],b||c[g]||(c["!"+g]=1)};a.Eb=function(a){var b,d,f,e,g,h=0,l,k="",m="";if(a&&255<a.length&&(b=""+a,d=b.indexOf("?"),0<d&&(l=b.substring(d+1),b=b.substring(0,d),e=b.toLowerCase(),f=0,"http://"==e.substring(0,7)?f+=7:"https://"==e.substring(0,8)&&(f+=8),d=e.indexOf("/",f),0<d&&(e=e.substring(f,d),g=b.substring(d),b=b.substring(0,d),0<=e.indexOf("google")?h=",q,ie,start,search_key,word,kw,cd,":
0<=e.indexOf("yahoo.co")&&(h=",p,ei,"),h&&l)))){if((a=l.split("&"))&&1<a.length){for(f=0;f<a.length;f++)e=a[f],d=e.indexOf("="),0<d&&0<=h.indexOf(","+e.substring(0,d)+",")?k+=(k?"&":"")+e:m+=(m?"&":"")+e;k&&m?l=k+"&"+m:m=""}d=253-(l.length-m.length)-b.length;a=b+(0<d?g.substring(0,d):"")+"?"+l}return a};a.eb=function(c){var b=a.d.visibilityState,d=["webkitvisibilitychange","visibilitychange"];b||(b=a.d.webkitVisibilityState);if(b&&"prerender"==b){if(c)for(b=0;b<d.length;b++)a.d.addEventListener(d[b],
function(){var b=a.d.visibilityState;b||(b=a.d.webkitVisibilityState);"visible"==b&&c()});return!1}return!0};a.fa=!1;a.J=!1;a.ub=function(){a.J=!0;a.j()};a.da=!1;a.V=!1;a.pb=function(c){a.marketingCloudVisitorID=c;a.V=!0;a.j()};a.ga=!1;a.W=!1;a.vb=function(c){a.visitorOptedOut=c;a.W=!0;a.j()};a.aa=!1;a.S=!1;a.$a=function(c){a.analyticsVisitorID=c;a.S=!0;a.j()};a.ca=!1;a.U=!1;a.bb=function(c){a.audienceManagerLocationHint=c;a.U=!0;a.j()};a.ba=!1;a.T=!1;a.ab=function(c){a.audienceManagerBlob=c;a.T=
!0;a.j()};a.cb=function(c){a.maxDelay||(a.maxDelay=250);return a.p("_d")?(c&&setTimeout(function(){c()},a.maxDelay),!1):!0};a.ea=!1;a.I=!1;a.za=function(){a.I=!0;a.j()};a.isReadyToTrack=function(){var c=!0,b=a.visitor,d,f,e;a.fa||a.J||(a.eb(a.ub)?a.J=!0:a.fa=!0);if(a.fa&&!a.J)return!1;b&&b.isAllowed()&&(a.da||a.marketingCloudVisitorID||!b.getMarketingCloudVisitorID||(a.da=!0,a.marketingCloudVisitorID=b.getMarketingCloudVisitorID([a,a.pb]),a.marketingCloudVisitorID&&(a.V=!0)),a.ga||a.visitorOptedOut||
!b.isOptedOut||(a.ga=!0,a.visitorOptedOut=b.isOptedOut([a,a.vb]),a.visitorOptedOut!=p&&(a.W=!0)),a.aa||a.analyticsVisitorID||!b.getAnalyticsVisitorID||(a.aa=!0,a.analyticsVisitorID=b.getAnalyticsVisitorID([a,a.$a]),a.analyticsVisitorID&&(a.S=!0)),a.ca||a.audienceManagerLocationHint||!b.getAudienceManagerLocationHint||(a.ca=!0,a.audienceManagerLocationHint=b.getAudienceManagerLocationHint([a,a.bb]),a.audienceManagerLocationHint&&(a.U=!0)),a.ba||a.audienceManagerBlob||!b.getAudienceManagerBlob||(a.ba=
!0,a.audienceManagerBlob=b.getAudienceManagerBlob([a,a.ab]),a.audienceManagerBlob&&(a.T=!0)),c=a.da&&!a.V&&!a.marketingCloudVisitorID,b=a.aa&&!a.S&&!a.analyticsVisitorID,d=a.ca&&!a.U&&!a.audienceManagerLocationHint,f=a.ba&&!a.T&&!a.audienceManagerBlob,e=a.ga&&!a.W,c=c||b||d||f||e?!1:!0);a.ea||a.I||(a.cb(a.za)?a.I=!0:a.ea=!0);a.ea&&!a.I&&(c=!1);return c};a.o=p;a.u=0;a.callbackWhenReadyToTrack=function(c,b,d){var f;f={};f.zb=c;f.yb=b;f.wb=d;a.o==p&&(a.o=[]);a.o.push(f);0==a.u&&(a.u=setInterval(a.j,
100))};a.j=function(){var c;if(a.isReadyToTrack()&&(a.tb(),a.o!=p))for(;0<a.o.length;)c=a.o.shift(),c.yb.apply(c.zb,c.wb)};a.tb=function(){a.u&&(clearInterval(a.u),a.u=0)};a.mb=function(c){var b,d,f=p,e=p;if(!a.isReadyToTrack()){b=[];if(c!=p)for(d in f={},c)f[d]=c[d];e={};a.Ya(e,!0);b.push(f);b.push(e);a.callbackWhenReadyToTrack(a,a.track,b);return!0}return!1};a.Gb=function(){var c=a.cookieRead("s_fid"),b="",d="",f;f=8;var e=4;if(!c||0>c.indexOf("-")){for(c=0;16>c;c++)f=Math.floor(Math.random()*f),
b+="0123456789ABCDEF".substring(f,f+1),f=Math.floor(Math.random()*e),d+="0123456789ABCDEF".substring(f,f+1),f=e=16;c=b+"-"+d}a.cookieWrite("s_fid",c,1)||(c=0);return c};a.t=a.track=function(c,b){var d,f=new Date,e="s"+Math.floor(f.getTime()/108E5)%10+Math.floor(1E13*Math.random()),g=f.getYear(),g="t="+a.escape(f.getDate()+"/"+f.getMonth()+"/"+(1900>g?g+1900:g)+" "+f.getHours()+":"+f.getMinutes()+":"+f.getSeconds()+" "+f.getDay()+" "+f.getTimezoneOffset());a.visitor&&a.visitor.getAuthState&&(a.authState=
a.visitor.getAuthState());a.p("_s");a.mb(c)||(b&&a.R(b),c&&(d={},a.Ya(d,0),a.R(c)),a.Mb()&&!a.visitorOptedOut&&(a.analyticsVisitorID||a.marketingCloudVisitorID||(a.fid=a.Gb()),a.Pb(),a.usePlugins&&a.doPlugins&&a.doPlugins(a),a.account&&(a.abort||(a.trackOffline&&!a.timestamp&&(a.timestamp=Math.floor(f.getTime()/1E3)),f=k.location,a.pageURL||(a.pageURL=f.href?f.href:f),a.referrer||a.Za||(f=a.Util.getQueryParam("adobe_mc_ref",null,null,!0),a.referrer=f||void 0===f?void 0===f?"":f:n.document.referrer),
a.Za=1,a.referrer=a.Eb(a.referrer),a.p("_g")),a.Jb()&&!a.abort&&(a.visitor&&!a.supplementalDataID&&a.visitor.getSupplementalDataID&&(a.supplementalDataID=a.visitor.getSupplementalDataID("AppMeasurement:"+a._in,a.expectSupplementalData?!1:!0)),a.Kb(),g+=a.Ib(),a.ob(e,g),a.p("_t"),a.referrer=""))),c&&a.R(d,1));a.abort=a.supplementalDataID=a.timestamp=a.pageURLRest=a.linkObject=a.clickObject=a.linkURL=a.linkName=a.linkType=k.s_objectID=a.pe=a.pev1=a.pev2=a.pev3=a.e=a.lightProfileID=0};a.Ba=[];a.registerPreTrackCallback=
function(c){for(var b=[],d=1;d<arguments.length;d++)b.push(arguments[d]);"function"==typeof c?a.Ba.push([c,b]):a.debugTracking&&a.F("DEBUG: Non function type passed to registerPreTrackCallback")};a.hb=function(c){a.xa(a.Ba,c)};a.Aa=[];a.registerPostTrackCallback=function(c){for(var b=[],d=1;d<arguments.length;d++)b.push(arguments[d]);"function"==typeof c?a.Aa.push([c,b]):a.debugTracking&&a.F("DEBUG: Non function type passed to registerPostTrackCallback")};a.gb=function(c){a.xa(a.Aa,c)};a.xa=function(c,
b){if("object"==typeof c)for(var d=0;d<c.length;d++){var f=c[d][0],e=c[d][1];e.unshift(b);if("function"==typeof f)try{f.apply(null,e)}catch(g){a.debugTracking&&a.F(g.message)}}};a.tl=a.trackLink=function(c,b,d,f,e){a.linkObject=c;a.linkType=b;a.linkName=d;e&&(a.l=c,a.A=e);return a.track(f)};a.trackLight=function(c,b,d,f){a.lightProfileID=c;a.lightStoreForSeconds=b;a.lightIncrementBy=d;return a.track(f)};a.clearVars=function(){var c,b;for(c=0;c<a.g.length;c++)if(b=a.g[c],"prop"==b.substring(0,4)||
"eVar"==b.substring(0,4)||"hier"==b.substring(0,4)||"list"==b.substring(0,4)||"channel"==b||"events"==b||"eventList"==b||"products"==b||"productList"==b||"purchaseID"==b||"transactionID"==b||"state"==b||"zip"==b||"campaign"==b)a[b]=void 0};a.tagContainerMarker="";a.ob=function(c,b){var d=a.ib()+"/"+c+"?AQB=1&ndh=1&pf=1&"+(a.ya()?"callback=s_c_il["+a._in+"].doPostbacks&et=1&":"")+b+"&AQE=1";a.hb(d);a.fb(d);a.X()};a.ib=function(){var c=a.jb();return"http"+(a.ssl?"s":"")+"://"+c+"/b/ss/"+a.account+"/"+
(a.mobile?"5.":"")+(a.ya()?"10":"1")+"/JS-"+a.version+(a.Sb?"T":"")+(a.tagContainerMarker?"-"+a.tagContainerMarker:"")};a.ya=function(){return a.AudienceManagement&&a.AudienceManagement.isReady()||0!=a.usePostbacks};a.jb=function(){var c=a.dc,b=a.trackingServer;b?a.trackingServerSecure&&a.ssl&&(b=a.trackingServerSecure):(c=c?(""+c).toLowerCase():"d1","d1"==c?c="112":"d2"==c&&(c="122"),b=a.lb()+"."+c+".2o7.net");return b};a.lb=function(){var c=a.visitorNamespace;c||(c=a.account.split(",")[0],c=c.replace(/[^0-9a-z]/gi,
""));return c};a.Xa=/{(%?)(.*?)(%?)}/;a.Wb=RegExp(a.Xa.source,"g");a.Db=function(c){if("object"==typeof c.dests)for(var b=0;b<c.dests.length;++b){var d=c.dests[b];if("string"==typeof d.c&&"aa."==d.id.substr(0,3))for(var f=d.c.match(a.Wb),e=0;e<f.length;++e){var g=f[e],h=g.match(a.Xa),k="";"%"==h[1]&&"timezone_offset"==h[2]?k=(new Date).getTimezoneOffset():"%"==h[1]&&"timestampz"==h[2]&&(k=a.Hb());d.c=d.c.replace(g,a.escape(k))}}};a.Hb=function(){var c=new Date,b=new Date(6E4*Math.abs(c.getTimezoneOffset()));
return a.k(4,c.getFullYear())+"-"+a.k(2,c.getMonth()+1)+"-"+a.k(2,c.getDate())+"T"+a.k(2,c.getHours())+":"+a.k(2,c.getMinutes())+":"+a.k(2,c.getSeconds())+(0<c.getTimezoneOffset()?"-":"+")+a.k(2,b.getUTCHours())+":"+a.k(2,b.getUTCMinutes())};a.k=function(a,b){return(Array(a+1).join(0)+b).slice(-a)};a.ua={};a.doPostbacks=function(c){if("object"==typeof c)if(a.Db(c),"object"==typeof a.AudienceManagement&&"function"==typeof a.AudienceManagement.isReady&&a.AudienceManagement.isReady()&&"function"==typeof a.AudienceManagement.passData)a.AudienceManagement.passData(c);
else if("object"==typeof c&&"object"==typeof c.dests)for(var b=0;b<c.dests.length;++b){var d=c.dests[b];"object"==typeof d&&"string"==typeof d.c&&"string"==typeof d.id&&"aa."==d.id.substr(0,3)&&(a.ua[d.id]=new Image,a.ua[d.id].alt="",a.ua[d.id].src=d.c)}};a.fb=function(c){a.i||a.Lb();a.i.push(c);a.ma=a.C();a.Va()};a.Lb=function(){a.i=a.Nb();a.i||(a.i=[])};a.Nb=function(){var c,b;if(a.ta()){try{(b=k.localStorage.getItem(a.qa()))&&(c=k.JSON.parse(b))}catch(d){}return c}};a.ta=function(){var c=!0;a.trackOffline&&
a.offlineFilename&&k.localStorage&&k.JSON||(c=!1);return c};a.La=function(){var c=0;a.i&&(c=a.i.length);a.q&&c++;return c};a.X=function(){if(a.q&&(a.B&&a.B.complete&&a.B.G&&a.B.wa(),a.q))return;a.Ma=p;if(a.ra)a.ma>a.O&&a.Ta(a.i),a.va(500);else{var c=a.xb();if(0<c)a.va(c);else if(c=a.Ia())a.q=1,a.Ob(c),a.Rb(c)}};a.va=function(c){a.Ma||(c||(c=0),a.Ma=setTimeout(a.X,c))};a.xb=function(){var c;if(!a.trackOffline||0>=a.offlineThrottleDelay)return 0;c=a.C()-a.Ra;return a.offlineThrottleDelay<c?0:a.offlineThrottleDelay-
c};a.Ia=function(){if(0<a.i.length)return a.i.shift()};a.Ob=function(c){if(a.debugTracking){var b="AppMeasurement Debug: "+c;c=c.split("&");var d;for(d=0;d<c.length;d++)b+="\n\t"+a.unescape(c[d]);a.F(b)}};a.nb=function(){return a.marketingCloudVisitorID||a.analyticsVisitorID};a.Z=!1;var t;try{t=JSON.parse('{"x":"y"}')}catch(w){t=null}t&&"y"==t.x?(a.Z=!0,a.Y=function(a){return JSON.parse(a)}):k.$&&k.$.parseJSON?(a.Y=function(a){return k.$.parseJSON(a)},a.Z=!0):a.Y=function(){return null};a.Rb=function(c){var b,
d,f;a.nb()&&2047<c.length&&("undefined"!=typeof XMLHttpRequest&&(b=new XMLHttpRequest,"withCredentials"in b?d=1:b=0),b||"undefined"==typeof XDomainRequest||(b=new XDomainRequest,d=2),b&&(a.AudienceManagement&&a.AudienceManagement.isReady()||0!=a.usePostbacks)&&(a.Z?b.Da=!0:b=0));!b&&a.Wa&&(c=c.substring(0,2047));!b&&a.d.createElement&&(0!=a.usePostbacks||a.AudienceManagement&&a.AudienceManagement.isReady())&&(b=a.d.createElement("SCRIPT"))&&"async"in b&&((f=(f=a.d.getElementsByTagName("HEAD"))&&f[0]?
f[0]:a.d.body)?(b.type="text/javascript",b.setAttribute("async","async"),d=3):b=0);b||(b=new Image,b.alt="",b.abort||"undefined"===typeof k.InstallTrigger||(b.abort=function(){b.src=p}));b.Sa=Date.now();b.Fa=function(){try{b.G&&(clearTimeout(b.G),b.G=0)}catch(a){}};b.onload=b.wa=function(){b.Sa&&(a.na=Date.now()-b.Sa);a.gb(c);b.Fa();a.Bb();a.ha();a.q=0;a.X();if(b.Da){b.Da=!1;try{a.doPostbacks(a.Y(b.responseText))}catch(d){}}};b.onabort=b.onerror=b.Ja=function(){b.Fa();(a.trackOffline||a.ra)&&a.q&&
a.i.unshift(a.Ab);a.q=0;a.ma>a.O&&a.Ta(a.i);a.ha();a.va(500)};b.onreadystatechange=function(){4==b.readyState&&(200==b.status?b.wa():b.Ja())};a.Ra=a.C();if(1==d||2==d){var e=c.indexOf("?");f=c.substring(0,e);e=c.substring(e+1);e=e.replace(/&callback=[a-zA-Z0-9_.\[\]]+/,"");1==d?(b.open("POST",f,!0),b.send(e)):2==d&&(b.open("POST",f),b.send(e))}else if(b.src=c,3==d){if(a.Pa)try{f.removeChild(a.Pa)}catch(g){}f.firstChild?f.insertBefore(b,f.firstChild):f.appendChild(b);a.Pa=a.B}b.G=setTimeout(function(){b.G&&
(b.complete?b.wa():(a.trackOffline&&b.abort&&b.abort(),b.Ja()))},5E3);a.Ab=c;a.B=k["s_i_"+a.replace(a.account,",","_")]=b;if(a.useForcedLinkTracking&&a.K||a.A)a.forcedLinkTrackingTimeout||(a.forcedLinkTrackingTimeout=250),a.ia=setTimeout(a.ha,a.forcedLinkTrackingTimeout)};a.Bb=function(){if(a.ta()&&!(a.Qa>a.O))try{k.localStorage.removeItem(a.qa()),a.Qa=a.C()}catch(c){}};a.Ta=function(c){if(a.ta()){a.Va();try{k.localStorage.setItem(a.qa(),k.JSON.stringify(c)),a.O=a.C()}catch(b){}}};a.Va=function(){if(a.trackOffline){if(!a.offlineLimit||
0>=a.offlineLimit)a.offlineLimit=10;for(;a.i.length>a.offlineLimit;)a.Ia()}};a.forceOffline=function(){a.ra=!0};a.forceOnline=function(){a.ra=!1};a.qa=function(){return a.offlineFilename+"-"+a.visitorNamespace+a.account};a.C=function(){return(new Date).getTime()};a.Na=function(a){a=a.toLowerCase();return 0!=a.indexOf("#")&&0!=a.indexOf("about:")&&0!=a.indexOf("opera:")&&0!=a.indexOf("javascript:")?!0:!1};a.setTagContainer=function(c){var b,d,f;a.Sb=c;for(b=0;b<a._il.length;b++)if((d=a._il[b])&&"s_l"==
d._c&&d.tagContainerName==c){a.R(d);if(d.lmq)for(b=0;b<d.lmq.length;b++)f=d.lmq[b],a.loadModule(f.n);if(d.ml)for(f in d.ml)if(a[f])for(b in c=a[f],f=d.ml[f],f)!Object.prototype[b]&&("function"!=typeof f[b]||0>(""+f[b]).indexOf("s_c_il"))&&(c[b]=f[b]);if(d.mmq)for(b=0;b<d.mmq.length;b++)f=d.mmq[b],a[f.m]&&(c=a[f.m],c[f.f]&&"function"==typeof c[f.f]&&(f.a?c[f.f].apply(c,f.a):c[f.f].apply(c)));if(d.tq)for(b=0;b<d.tq.length;b++)a.track(d.tq[b]);d.s=a;break}};a.Util={urlEncode:a.escape,urlDecode:a.unescape,
cookieRead:a.cookieRead,cookieWrite:a.cookieWrite,getQueryParam:function(c,b,d,f){var e,g="";b||(b=a.pageURL?a.pageURL:k.location);d=d?d:"&";if(!c||!b)return g;b=""+b;e=b.indexOf("?");if(0>e)return g;b=d+b.substring(e+1)+d;if(!f||!(0<=b.indexOf(d+c+d)||0<=b.indexOf(d+c+"="+d))){e=b.indexOf("#");0<=e&&(b=b.substr(0,e)+d);e=b.indexOf(d+c+"=");if(0>e)return g;b=b.substring(e+d.length+c.length+1);e=b.indexOf(d);0<=e&&(b=b.substring(0,e));0<b.length&&(g=a.unescape(b));return g}},getIeVersion:function(){if(document.documentMode)return document.documentMode;
for(var a=7;4<a;a--){var b=document.createElement("div");b.innerHTML="\x3c!--[if IE "+a+"]><span></span><![endif]--\x3e";if(b.getElementsByTagName("span").length)return a}return null}};a.H="supplementalDataID timestamp dynamicVariablePrefix visitorID marketingCloudVisitorID analyticsVisitorID audienceManagerLocationHint authState fid vmk visitorMigrationKey visitorMigrationServer visitorMigrationServerSecure charSet visitorNamespace cookieDomainPeriods fpCookieDomainPeriods cookieLifetime pageName pageURL customerPerspective referrer contextData currencyCode lightProfileID lightStoreForSeconds lightIncrementBy retrieveLightProfiles deleteLightProfiles retrieveLightData".split(" ");
a.g=a.H.concat("purchaseID variableProvider channel server pageType transactionID campaign state zip events events2 products audienceManagerBlob tnt".split(" "));a.oa="timestamp charSet visitorNamespace cookieDomainPeriods cookieLifetime contextData lightProfileID lightStoreForSeconds lightIncrementBy".split(" ");a.P=a.oa.slice(0);a.Ca="account allAccounts debugTracking visitor visitorOptedOut trackOffline offlineLimit offlineThrottleDelay offlineFilename usePlugins doPlugins configURL visitorSampling visitorSamplingGroup linkObject clickObject linkURL linkName linkType trackDownloadLinks trackExternalLinks trackClickMap trackInlineStats linkLeaveQueryString linkTrackVars linkTrackEvents linkDownloadFileTypes linkExternalFilters linkInternalFilters useForcedLinkTracking forcedLinkTrackingTimeout trackingServer trackingServerSecure ssl abort mobile dc lightTrackVars maxDelay expectSupplementalData usePostbacks registerPreTrackCallback registerPostTrackCallback AudienceManagement".split(" ");
for(m=0;250>=m;m++)76>m&&(a.g.push("prop"+m),a.P.push("prop"+m)),a.g.push("eVar"+m),a.P.push("eVar"+m),6>m&&a.g.push("hier"+m),4>m&&a.g.push("list"+m);m="pe pev1 pev2 pev3 latitude longitude resolution colorDepth javascriptVersion javaEnabled cookiesEnabled browserWidth browserHeight connectionType homepage pageURLRest marketingCloudOrgID".split(" ");a.g=a.g.concat(m);a.H=a.H.concat(m);a.ssl=0<=k.location.protocol.toLowerCase().indexOf("https");a.charSet="UTF-8";a.contextData={};a.offlineThrottleDelay=
0;a.offlineFilename="AppMeasurement.offline";a.Ra=0;a.ma=0;a.O=0;a.Qa=0;a.linkDownloadFileTypes="exe,zip,wav,mp3,mov,mpg,avi,wmv,pdf,doc,docx,xls,xlsx,ppt,pptx";a.w=k;a.d=k.document;try{if(a.Wa=!1,navigator){var v=navigator.userAgent;if("Microsoft Internet Explorer"==navigator.appName||0<=v.indexOf("MSIE ")||0<=v.indexOf("Trident/")&&0<=v.indexOf("Windows NT 6"))a.Wa=!0}}catch(x){}a.ha=function(){a.ia&&(k.clearTimeout(a.ia),a.ia=p);a.l&&a.K&&a.l.dispatchEvent(a.K);a.A&&("function"==typeof a.A?a.A():
a.l&&a.l.href&&(a.d.location=a.l.href));a.l=a.K=a.A=0};a.Ua=function(){a.b=a.d.body;a.b?(a.v=function(c){var b,d,f,e,g;if(!(a.d&&a.d.getElementById("cppXYctnr")||c&&c["s_fe_"+a._in])){if(a.Ea)if(a.useForcedLinkTracking)a.b.removeEventListener("click",a.v,!1);else{a.b.removeEventListener("click",a.v,!0);a.Ea=a.useForcedLinkTracking=0;return}else a.useForcedLinkTracking=0;a.clickObject=c.srcElement?c.srcElement:c.target;try{if(!a.clickObject||a.N&&a.N==a.clickObject||!(a.clickObject.tagName||a.clickObject.parentElement||
a.clickObject.parentNode))a.clickObject=0;else{var h=a.N=a.clickObject;a.la&&(clearTimeout(a.la),a.la=0);a.la=setTimeout(function(){a.N==h&&(a.N=0)},1E4);f=a.La();a.track();if(f<a.La()&&a.useForcedLinkTracking&&c.target){for(e=c.target;e&&e!=a.b&&"A"!=e.tagName.toUpperCase()&&"AREA"!=e.tagName.toUpperCase();)e=e.parentNode;if(e&&(g=e.href,a.Na(g)||(g=0),d=e.target,c.target.dispatchEvent&&g&&(!d||"_self"==d||"_top"==d||"_parent"==d||k.name&&d==k.name))){try{b=a.d.createEvent("MouseEvents")}catch(l){b=
new k.MouseEvent}if(b){try{b.initMouseEvent("click",c.bubbles,c.cancelable,c.view,c.detail,c.screenX,c.screenY,c.clientX,c.clientY,c.ctrlKey,c.altKey,c.shiftKey,c.metaKey,c.button,c.relatedTarget)}catch(m){b=0}b&&(b["s_fe_"+a._in]=b.s_fe=1,c.stopPropagation(),c.stopImmediatePropagation&&c.stopImmediatePropagation(),c.preventDefault(),a.l=c.target,a.K=b)}}}}}catch(n){a.clickObject=0}}},a.b&&a.b.attachEvent?a.b.attachEvent("onclick",a.v):a.b&&a.b.addEventListener&&(navigator&&(0<=navigator.userAgent.indexOf("WebKit")&&
a.d.createEvent||0<=navigator.userAgent.indexOf("Firefox/2")&&k.MouseEvent)&&(a.Ea=1,a.useForcedLinkTracking=1,a.b.addEventListener("click",a.v,!0)),a.b.addEventListener("click",a.v,!1))):setTimeout(a.Ua,30)};a.Cb();a.ac||(r?a.setAccount(r):a.F("Error, missing Report Suite ID in AppMeasurement initialization"),a.Ua(),a.loadModule("ActivityMap"))}
function s_gi(r){var a,k=window.s_c_il,p,n,m=r.split(","),s,u,t=0;if(k)for(p=0;!t&&p<k.length;){a=k[p];if("s_c"==a._c&&(a.account||a.oun))if(a.account&&a.account==r)t=1;else for(n=a.account?a.account:a.oun,n=a.allAccounts?a.allAccounts:n.split(","),s=0;s<m.length;s++)for(u=0;u<n.length;u++)m[s]==n[u]&&(t=1);p++}t?a.setAccount&&a.setAccount(r):a=new AppMeasurement(r);return a}AppMeasurement.getInstance=s_gi;window.s_objectID||(window.s_objectID=0);
function s_pgicq(){var r=window,a=r.s_giq,k,p,n;if(a)for(k=0;k<a.length;k++)p=a[k],n=s_gi(p.oun),n.setAccount(p.un),n.setTagContainer(p.tagContainerName);r.s_giq=0}s_pgicq();

