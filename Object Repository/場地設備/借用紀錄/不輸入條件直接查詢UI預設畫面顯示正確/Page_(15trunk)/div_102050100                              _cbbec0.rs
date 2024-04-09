<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_102050100                              _cbbec0</name>
   <tag></tag>
   <elementGuidId>85d2a92f-6bc2-48b0-bb1c-d0a41ce1a210</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='nrResults']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#nrResults</value>
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
      <value>div</value>
      <webElementGuid>cb5374c1-ad1d-47ed-b1f5-7638aa393c3c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>nrResults</value>
      <webElementGuid>0a21b255-bb21-4a23-92bf-5ef337e1d306</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>pages w l pages_border_top</value>
      <webElementGuid>55f82851-192c-4f5d-acef-54847d647aba</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
		
      	 
			
10
20
50
100

 		 
		 
			
	             
	             
	               59
	  			   筆
	              
				 (0.78s) •
	
			 
	        
	
				
  

//&lt;![CDATA[

  	var exMsg='頁碼錯誤，請重新輸入'
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == 'ro')
          Tapestry.submit_form('CautareRapida', 'langFirst');
      if (document.CautareRapida.languages.value == 'ru')
          Tapestry.submit_form('CautareRapida', 'langSecond');
      if (document.CautareRapida.languages.value == 'en')
          Tapestry.submit_form('CautareRapida', 'langThird');
      if (document.CautareRapida.languages.value == 'ch')
          Tapestry.submit_form('CautareRapida', 'langFourth');
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = '1';
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									
									  
									  
									   
									     2
									   	
									               
									
									  
									  
									   
									     3
									   	
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						      
			
		</value>
      <webElementGuid>1136572c-fe88-449b-a778-03021b6dbab0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;nrResults&quot;)</value>
      <webElementGuid>56607ce2-af51-4e47-9e6e-408fd84eb17f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='nrResults']</value>
      <webElementGuid>e522b02b-893c-47a6-aba5-f126aefddc33</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='searchForm']/div[6]/div</value>
      <webElementGuid>f3bc6129-4f6e-4e02-82d6-bfa74884c704</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Ⅹ'])[1]/following::div[2]</value>
      <webElementGuid>2c6e0762-ac56-4918-9184-35fd74aaf769</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Ⅸ'])[1]/following::div[2]</value>
      <webElementGuid>58c59761-ab15-4583-8014-21ba306a5880</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[6]/div</value>
      <webElementGuid>5e8ea418-19f5-499b-aa2a-fe8e936a7f55</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'nrResults' and (text() = concat(&quot;
		
      	 
			
10
20
50
100

 		 
		 
			
	             
	             
	               59
	  			   筆
	              
				 (0.78s) •
	
			 
	        
	
				
  

//&lt;![CDATA[

  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									
									  
									  
									   
									     2
									   	
									               
									
									  
									  
									   
									     3
									   	
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						      
			
		&quot;) or . = concat(&quot;
		
      	 
			
10
20
50
100

 		 
		 
			
	             
	             
	               59
	  			   筆
	              
				 (0.78s) •
	
			 
	        
	
				
  

//&lt;![CDATA[

  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
//]]&gt;


   


//&lt;![CDATA[

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
//]]&gt;


						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									
									  
									  
									   
									     2
									   	
									               
									
									  
									  
									   
									     3
									   	
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						      
			
		&quot;))]</value>
      <webElementGuid>3689bff6-9529-437d-a90f-503a000e4015</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
