<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_DetailsdjConfig  baseRelativePathinspi_19211e</name>
   <tag></tag>
   <elementGuidId>009a6c2d-6109-4155-a47c-83e013a38c14</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//html</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>db39a8ae-94f2-41b9-9c28-67cc2eb557d6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding='UTF-8';


































&lt;!--
if (typeof window.event == 'undefined'){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == 'INPUT' &amp;&amp; test_type == 'TEXT') ||(test_var == 'INPUT' &amp;&amp; test_type == 'PASSWORD')|| test_var == 'TEXTAREA'){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == 'INPUT' &amp;&amp; test_type == 'TEXT') ||(test_var == 'INPUT' &amp;&amp; test_type == 'PASSWORD') || test_var == 'TEXTAREA'){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
var calendar_startDatePicker;
var calendar_endDatePicker;
function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==''){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/', closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/');
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
		交易明細
	 
		  
		  
		  
		    
			
			
		  
		


	
	     
	
		function refreshLoandesk() {
			var opener = window.opener;
			if (opener != null) {
				var document1 = opener.document;
				if (document1 != null) {
					var button1 = document1.getElementById(&quot;siteEquBorrowLink&quot;);
					if (button1 != null) {
						button1.click();
					}
				}
			}
		}
	
	
	
	 
 	












 		
 			
				使用狀態:
				
					
					已歸還
					
				
			
			
				讀者姓名:
				aaa1216
			
			
				讀者證號:
				AAA1216
			
			
				讀者身份類別-:
				01大學生一年級
			
			
				設備名稱:
				台中總館_3F視聽室001
			
			
				設備編號:
				AV3F001
			
			
				設備狀態:
				
					正常
				
			
			
				館藏地:
				台中總館
			
			
				應收費用:
				0
			
			
				使用人數:
				1
			
			
				借用用途:
				1
			
			
				借用器材或其他說明:
				1
			
			
				流通註記:
				測試202403181535
			
			
				預約時間:
				
					
						2021/05/04 14:58:00
					
				
			
			
				借用開始時間:
				
					 
					借閱時間-時:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					借閱時間-分:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				
			
			
				借用應還時間:
				
					 
					到期時間-時:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					到期時間-分:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				
			
			
				交易結束時間:
				2021/05/04 15:03:09
			
			
				執行交易館員:
				catc神資測試
			
			
				結束交易館員:
				catc神資測試
			
			
			
				
					
						修改/存檔
					
				
			
 		
 		
 		
 	

	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;Form&quot;);
calendar_startDatePicker = new Calendar(1620057600000);
	
calendar_startDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_startDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;Form&quot;).startDatePicker;
  var value = calendar_startDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
calendar_endDatePicker = new Calendar(1620143940000);
	
calendar_endDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_endDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;Form&quot;).endDatePicker;
  var value = calendar_endDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}

closeDialogComponent('CloseReminderDialog');
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById('ajaxLoaderDiv');
            	var processDiv = document.getElementById('pressingDiv');
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById('pressingDiv');
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent('TinreadErrorDialog');
closeDialogComponent('TinreadMessageDialog');

tapestry.form.focusField('TextArea');});
// -->




/html[1]&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六      12345678910111213141516171819202122232425262728293031     18 三月, 2024Clear&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六      12345678910111213141516171819202122232425262728293031     18 三月, 2024Clear</value>
      <webElementGuid>825eded7-5f77-4dd6-b627-1932a20dfbc0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>598bd419-ce54-4ddb-8879-10495189bd8d</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>241e2c62-261d-4dd3-bafd-8894c348d8eb</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;


































&lt;!--
if (typeof window.event == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)|| test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;) || test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
var calendar_startDatePicker;
var calendar_endDatePicker;
function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
		交易明細
	 
		  
		  
		  
		    
			
			
		  
		


	
	     
	
		function refreshLoandesk() {
			var opener = window.opener;
			if (opener != null) {
				var document1 = opener.document;
				if (document1 != null) {
					var button1 = document1.getElementById(&quot;siteEquBorrowLink&quot;);
					if (button1 != null) {
						button1.click();
					}
				}
			}
		}
	
	
	
	 
 	












 		
 			
				使用狀態:
				
					
					已歸還
					
				
			
			
				讀者姓名:
				aaa1216
			
			
				讀者證號:
				AAA1216
			
			
				讀者身份類別-:
				01大學生一年級
			
			
				設備名稱:
				台中總館_3F視聽室001
			
			
				設備編號:
				AV3F001
			
			
				設備狀態:
				
					正常
				
			
			
				館藏地:
				台中總館
			
			
				應收費用:
				0
			
			
				使用人數:
				1
			
			
				借用用途:
				1
			
			
				借用器材或其他說明:
				1
			
			
				流通註記:
				測試202403181535
			
			
				預約時間:
				
					
						2021/05/04 14:58:00
					
				
			
			
				借用開始時間:
				
					 
					借閱時間-時:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					借閱時間-分:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				
			
			
				借用應還時間:
				
					 
					到期時間-時:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					到期時間-分:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				
			
			
				交易結束時間:
				2021/05/04 15:03:09
			
			
				執行交易館員:
				catc神資測試
			
			
				結束交易館員:
				catc神資測試
			
			
			
				
					
						修改/存檔
					
				
			
 		
 		
 		
 	

	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;Form&quot;);
calendar_startDatePicker = new Calendar(1620057600000);
	
calendar_startDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_startDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;Form&quot;).startDatePicker;
  var value = calendar_startDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
calendar_endDatePicker = new Calendar(1620143940000);
	
calendar_endDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_endDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;Form&quot;).endDatePicker;
  var value = calendar_endDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}

closeDialogComponent(&quot; , &quot;'&quot; , &quot;CloseReminderDialog&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextArea&quot; , &quot;'&quot; , &quot;);});
// -->




/html[1]&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六      12345678910111213141516171819202122232425262728293031     18 三月, 2024Clear&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六      12345678910111213141516171819202122232425262728293031     18 三月, 2024Clear&quot;) or . = concat(&quot;



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;


































&lt;!--
if (typeof window.event == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)|| test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;) || test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
var calendar_startDatePicker;
var calendar_endDatePicker;
function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
		交易明細
	 
		  
		  
		  
		    
			
			
		  
		


	
	     
	
		function refreshLoandesk() {
			var opener = window.opener;
			if (opener != null) {
				var document1 = opener.document;
				if (document1 != null) {
					var button1 = document1.getElementById(&quot;siteEquBorrowLink&quot;);
					if (button1 != null) {
						button1.click();
					}
				}
			}
		}
	
	
	
	 
 	












 		
 			
				使用狀態:
				
					
					已歸還
					
				
			
			
				讀者姓名:
				aaa1216
			
			
				讀者證號:
				AAA1216
			
			
				讀者身份類別-:
				01大學生一年級
			
			
				設備名稱:
				台中總館_3F視聽室001
			
			
				設備編號:
				AV3F001
			
			
				設備狀態:
				
					正常
				
			
			
				館藏地:
				台中總館
			
			
				應收費用:
				0
			
			
				使用人數:
				1
			
			
				借用用途:
				1
			
			
				借用器材或其他說明:
				1
			
			
				流通註記:
				測試202403181535
			
			
				預約時間:
				
					
						2021/05/04 14:58:00
					
				
			
			
				借用開始時間:
				
					 
					借閱時間-時:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					借閱時間-分:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				
			
			
				借用應還時間:
				
					 
					到期時間-時:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

					到期時間-分:
					
00
01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

				
			
			
				交易結束時間:
				2021/05/04 15:03:09
			
			
				執行交易館員:
				catc神資測試
			
			
				結束交易館員:
				catc神資測試
			
			
			
				
					
						修改/存檔
					
				
			
 		
 		
 		
 	

	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;Form&quot;);
calendar_startDatePicker = new Calendar(1620057600000);
	
calendar_startDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_startDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;Form&quot;).startDatePicker;
  var value = calendar_startDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
calendar_endDatePicker = new Calendar(1620143940000);
	
calendar_endDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_endDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;Form&quot;).endDatePicker;
  var value = calendar_endDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}

closeDialogComponent(&quot; , &quot;'&quot; , &quot;CloseReminderDialog&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextArea&quot; , &quot;'&quot; , &quot;);});
// -->




/html[1]&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六      12345678910111213141516171819202122232425262728293031     18 三月, 2024Clear&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六      12345678910111213141516171819202122232425262728293031     18 三月, 2024Clear&quot;))]</value>
      <webElementGuid>42f46210-3348-450d-9e37-5295eedb0454</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
