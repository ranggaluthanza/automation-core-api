<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>list</name>
   <tag></tag>
   <elementGuidId>afb776fe-f031-47cb-b4b5-4bb8b66a8054</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;pagination\&quot;: {\n\t\t\&quot;limit\&quot;: 10,\n\t\t\&quot;page\&quot;: 1,\n\t\t\&quot;column\&quot;: \&quot;id\&quot;,\n\t\t\&quot;ascending\&quot;: false,\n\t\t\&quot;query\&quot;: \&quot;\&quot;\n\t}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJqb2pvbm9taWMtand0LXNlcnZpY2UiLCJpYXQiOjE1NzA2Mjc1NzUsImV4cCI6MTYwMjE2MzU3NSwic3ViIjoxMzQ1OCwic2Vzc19pZCI6Njc5MjAsInVzZXIiOnsiaWQiOjEzNDU4LCJlbWFpbCI6Impvam9xYUBtYWlsaW5hdG9yLmNvbSIsImNvbXBhbnlfaWQiOjQxNCwidXNlcl9jb21wYW55X2lkIjoyMzUzfSwic2Vzc2lvbl9zZXR0aW5nIjoxfQ.PrS7y3xGU1kDpvpaKfwEgn2UOyBEnVWm9dwQ6Vfy7tA</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/list</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.urlAPI</defaultValue>
      <description></description>
      <id>2d2a97d3-51fa-4a9c-b6d7-ec7e1411c69d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
