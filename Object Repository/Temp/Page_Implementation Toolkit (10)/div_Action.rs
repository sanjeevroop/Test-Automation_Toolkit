<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Action</name>
   <tag></tag>
   <elementGuidId>e260d8e5-c82c-4cb7-b5b6-eea7bf24bea0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>pane-hScroll</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            
                
                    
                        Action
                        Deduction Number
                        Deduction Short Description
                        Deduction Long Description
                        Permanent / Variable /Both
                        Input Type
                        Deductible
                        Deduction Percentage
                        Linked Accumulator for Percentage
                        Deduction Type
                        Charity Aid Reference
                        Balance Type
                        Leaver Balance Recovery 
                        Display Balance on Payslip?
                        Display Deduction on Payslip?            
                        Clear down Tax Period                       
                    
                
            


            
                
                            
        (function($) {
            $.fn.swhgLoad = function(url, containerId, callback) {
                url = url + (url.indexOf('?') == -1 ? '?' : '&amp;') + '__swhg=' + new Date().getTime();

                $('&lt;div/>').load(url + ' ' + containerId, function(data, status, xhr) {
                    $(containerId).replaceWith($(this).html());
                    if (typeof(callback) === 'function') {
                        callback.apply(this, arguments);
                    }
                });
                return this;
            }

            $(function() {
                $('table[data-swhgajax=&quot;true&quot;],span[data-swhgajax=&quot;true&quot;]').each(function() {
                    var self = $(this);
                    var containerId = '#' + self.data('swhgcontainer');
                    var callback = getFunction(self.data('swhgcallback'));

                    $(containerId).parent().delegate(containerId + ' a[data-swhglnk=&quot;true&quot;]', 'click', function() {
                        $(containerId).swhgLoad($(this).attr('href'), containerId, callback);
                        return false;
                    });
                })
            });

            function getFunction(code, argNames) {
                argNames = argNames || [];
                var fn = window, parts = (code || &quot;&quot;).split(&quot;.&quot;);
                while (fn &amp;&amp; parts.length) {
                    fn = fn[parts.shift()];
                }
                if (typeof (fn) === &quot;function&quot;) {
                    return fn;
                }
                argNames.push(code);
                return Function.constructor.apply(null, argNames);
            }
        })(jQuery);
        
    
    
        
            
Action            
            
Deduction Number            
            
Deduction Short Description            
            
Deduction Long Description            
            
Permanent / Variable / Both            
            
Input Type            
            
Deductible            
            
Deduction Percentage            
            
Linked Accumulator for Percentage            
            
Deduction Type            
            
Charity Aid Reference            
            
Balance Type            
            
Leaver Balance Recovery            
            
Display Balance on Payslip?            
            
Display Deduction on Payslip?            
            
Clear down Tax Period            
        
    
     Save  Cancel Please Select PermanentVariableBoth     CashHours - Eligibility Only % Percentage Value  Yes No None EA01 EA02EA03EA04EA05EA06EA07EA08EA09EA10EA11EA12EA13EA14EA15EA16EA17EA18EA19EA20  Pre Tax And NI Pre Tax Non charity  Pennies from Heaven From Net Charity      None Increasing  Reducing         No Yes     No Yes         Yes No      
        
             
         Edit
         Delete
         Save
         Cancel 
        
        


            
        
            D11
         
    
            
        
            1111111
         
    
            
        
            11111111111111111
         
    
            
        
            Permanent
        Permanent
Variable
Both

    
            
        
            Cash
        Cash
Hours - Eligibility Only
%
Percentage
Value

    
             
            Yes
        Yes
No

    
            
        
            111
         
    
            
        
            None
        None
EA01
EA02
EA03
EA04
EA05
EA06
EA07
EA08
EA09
EA010
EA011
EA012
EA013
EA014
EA015
EA016
EA017
EA018
EA019
EA020

    
            
        
            From Net
        Pre Tax &amp; NI
Pre Tax Non-charity
Pennies from Heaven
From Net
Charity

    
            
        
            11111
         
    
            
        
            None
        Increasing
None
Reducing

    
             
                No
            Yes
No

        
             
            No
        Yes
No

    
             
        Yes
    Yes
No

    
            
        
            1
         
    
        
        
             
         Edit
         Delete
         Save
         Cancel 
        
        


            
        
            D13
         
    
            
        
            Test1
         
    
            
        
            Automation2
         
    
            
        
            Both
        Permanent
Variable
Both

    
            
        
            %
        Cash
Hours - Eligibility Only
%
Percentage
Value

    
             
            Yes
        Yes
No

    
            
        
            333
         
    
            
        
            EA020
        None
EA01
EA02
EA03
EA04
EA05
EA06
EA07
EA08
EA09
EA010
EA011
EA012
EA013
EA014
EA015
EA016
EA017
EA018
EA019
EA020

    
            
        
            Pre Tax &amp; NI
        Pre Tax &amp; NI
Pre Tax Non-charity
Pennies from Heaven
From Net
Charity

    
            
        
            111
         
    
            
        
            Increasing
        Increasing
None
Reducing

    
             
                No
            Yes
No

        
             
            No
        Yes
No

    
             
        Yes
    Yes
No

    
            
        
            02
         
    
        
        
             
         Edit
         Delete
         Save
         Cancel 
        
        


            
        
            D18
         
    
            
        
            Test18
         
    
            
        
            Automation18
         
    
            
        
            Both
        Permanent
Variable
Both

    
            
        
            %
        Cash
Hours - Eligibility Only
%
Percentage
Value

    
             
            Yes
        Yes
No

    
            
        
            5
         
    
            
        
            EA020
        None
EA01
EA02
EA03
EA04
EA05
EA06
EA07
EA08
EA09
EA010
EA011
EA012
EA013
EA014
EA015
EA016
EA017
EA018
EA019
EA020

    
            
        
            Pre Tax &amp; NI
        Pre Tax &amp; NI
Pre Tax Non-charity
Pennies from Heaven
From Net
Charity

    
            
        
            345
         
    
            
        
            Increasing
        Increasing
None
Reducing

    
             
                No
            Yes
No

        
             
            No
        Yes
No

    
             
        Yes
    Yes
No

    
            
        
            2
         
    
        
        
             
         Edit
         Delete
         Save
         Cancel 
        
        


            
        
            D12
         
    
            
        
            Test
         
    
            
        
            Automation1
         
    
            
        
            Variable
        Permanent
Variable
Both

    
            
        
            Percentage
        Cash
Hours - Eligibility Only
%
Percentage
Value

    
             
            No
        Yes
No

    
            
        
            3
         
    
            
        
            EA02
        None
EA01
EA02
EA03
EA04
EA05
EA06
EA07
EA08
EA09
EA010
EA011
EA012
EA013
EA014
EA015
EA016
EA017
EA018
EA019
EA020

    
            
        
            Pre Tax &amp; NI
        Pre Tax &amp; NI
Pre Tax Non-charity
Pennies from Heaven
From Net
Charity

    
            
        
            11
         
    
            
        
            Reducing
        Increasing
None
Reducing

    
             
                No
            Yes
No

        
             
            No
        Yes
No

    
             
        Yes
    Yes
No

    
            
        
            02
         
    
        
    
    

                

            
        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;divPartialView&quot;)/div[2]/div[@class=&quot;pane pane--table1&quot;]/div[@class=&quot;pane-hScroll&quot;]</value>
   </webElementProperties>
</WebElementEntity>
