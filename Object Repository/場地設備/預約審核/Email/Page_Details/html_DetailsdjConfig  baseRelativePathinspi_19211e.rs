<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_DetailsdjConfig  baseRelativePathinspi_19211e</name>
   <tag></tag>
   <elementGuidId>acb93aaf-3d3f-48c1-ae0d-4634fc25d9a6</elementGuidId>
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
      <webElementGuid>31563e0b-15b5-4db9-9d44-9da5fa45e04f</webElementGuid>
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

 

	

	
		
		  
		    
	電子郵件信箱(發送E-mail通知)
 
		  
		  
		  
		    
			
			
		  
		


	
	     

 

 
  
   
     







       
         
           
             
               
             
           
         
       


       
         
           主旨:
           
         
         
           收件人:
           
         
       

       
         
           
              
           
         
       

      

         
         
           
             
		         
        		   
		             發送        
        		   
        		 
             
           
         
       

     
     

       
       
       
       
        Sending...
       
       

       
      
       
      
   
  
 


	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;EmailForm&quot;);

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

tapestry.form.focusField('TextField');});
// -->





/html[1]window.FCKeditorAPI = {Version : &quot;2.6.3&quot;,VersionBuild : &quot;19836&quot;,Instances : new Object(),GetInstance : function( name ){return this.Instances[ name ];},_FormSubmit : function(){for ( var name in FCKeditorAPI.Instances ){var oEditor = FCKeditorAPI.Instances[ name ] ;if ( oEditor.GetParentForm &amp;&amp; oEditor.GetParentForm() == this )oEditor.UpdateLinkedField() ;}this._FCKOriginalSubmit() ;},_FunctionQueue	: {Functions : new Array(),IsRunning : false,Add : function( f ){this.Functions.push( f );if ( !this.IsRunning )this.StartNext();},StartNext : function(){var aQueue = this.Functions ;if ( aQueue.length > 0 ){this.IsRunning = true;aQueue[0].call();}else this.IsRunning = false;},Remove : function( f ){var aQueue = this.Functions;var i = 0, fFunc;while( (fFunc = aQueue[ i ]) ){if ( fFunc == f )aQueue.splice( i,1 );i++ ;}this.StartNext();}}}</value>
      <webElementGuid>5838a863-2af3-45b1-ac9e-9096173bc322</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>a63b8678-4ead-471b-9313-167a06d756c0</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>23ea3e03-25af-47d6-aec0-5e889ad9f5e2</webElementGuid>
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

 

	

	
		
		  
		    
	電子郵件信箱(發送E-mail通知)
 
		  
		  
		  
		    
			
			
		  
		


	
	     

 

 
  
   
     







       
         
           
             
               
             
           
         
       


       
         
           主旨:
           
         
         
           收件人:
           
         
       

       
         
           
              
           
         
       

      

         
         
           
             
		         
        		   
		             發送        
        		   
        		 
             
           
         
       

     
     

       
       
       
       
        Sending...
       
       

       
      
       
      
   
  
 


	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;EmailForm&quot;);

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

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextField&quot; , &quot;'&quot; , &quot;);});
// -->





/html[1]window.FCKeditorAPI = {Version : &quot;2.6.3&quot;,VersionBuild : &quot;19836&quot;,Instances : new Object(),GetInstance : function( name ){return this.Instances[ name ];},_FormSubmit : function(){for ( var name in FCKeditorAPI.Instances ){var oEditor = FCKeditorAPI.Instances[ name ] ;if ( oEditor.GetParentForm &amp;&amp; oEditor.GetParentForm() == this )oEditor.UpdateLinkedField() ;}this._FCKOriginalSubmit() ;},_FunctionQueue	: {Functions : new Array(),IsRunning : false,Add : function( f ){this.Functions.push( f );if ( !this.IsRunning )this.StartNext();},StartNext : function(){var aQueue = this.Functions ;if ( aQueue.length > 0 ){this.IsRunning = true;aQueue[0].call();}else this.IsRunning = false;},Remove : function( f ){var aQueue = this.Functions;var i = 0, fFunc;while( (fFunc = aQueue[ i ]) ){if ( fFunc == f )aQueue.splice( i,1 );i++ ;}this.StartNext();}}}&quot;) or . = concat(&quot;



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

 

	

	
		
		  
		    
	電子郵件信箱(發送E-mail通知)
 
		  
		  
		  
		    
			
			
		  
		


	
	     

 

 
  
   
     







       
         
           
             
               
             
           
         
       


       
         
           主旨:
           
         
         
           收件人:
           
         
       

       
         
           
              
           
         
       

      

         
         
           
             
		         
        		   
		             發送        
        		   
        		 
             
           
         
       

     
     

       
       
       
       
        Sending...
       
       

       
      
       
      
   
  
 


	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;EmailForm&quot;);

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

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextField&quot; , &quot;'&quot; , &quot;);});
// -->





/html[1]window.FCKeditorAPI = {Version : &quot;2.6.3&quot;,VersionBuild : &quot;19836&quot;,Instances : new Object(),GetInstance : function( name ){return this.Instances[ name ];},_FormSubmit : function(){for ( var name in FCKeditorAPI.Instances ){var oEditor = FCKeditorAPI.Instances[ name ] ;if ( oEditor.GetParentForm &amp;&amp; oEditor.GetParentForm() == this )oEditor.UpdateLinkedField() ;}this._FCKOriginalSubmit() ;},_FunctionQueue	: {Functions : new Array(),IsRunning : false,Add : function( f ){this.Functions.push( f );if ( !this.IsRunning )this.StartNext();},StartNext : function(){var aQueue = this.Functions ;if ( aQueue.length > 0 ){this.IsRunning = true;aQueue[0].call();}else this.IsRunning = false;},Remove : function( f ){var aQueue = this.Functions;var i = 0, fFunc;while( (fFunc = aQueue[ i ]) ){if ( fFunc == f )aQueue.splice( i,1 );i++ ;}this.StartNext();}}}&quot;))]</value>
      <webElementGuid>170c5072-7f91-47a6-869e-4a1a32c29fbb</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
