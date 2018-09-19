import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

//WebUI.openBrowser('')

//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3D4FV8yGK5DQOhIlvjWzHQcRoM_KEEirAcH8BFIajOoTF-v9ig3Gr-Cjafy5Lk_DzBTdmJ2xdh50zk9h1FHYZNQxykjENtFc7w5ptesd3bEn9c8ljC9wnrick23CUOke9sz-9H5wFIgLneBI3fNwY5dA&nonce=636703648449928387.MzgxOTIyYTQtYjNiMy00ZWUxLWE1YjMtMWNmNDhkYzdkYWI1YTJkODQyZmMtNjJjNS00ZjM1LWE1MDktZmI5MWQ0NDcyMzFj&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')

//WebUI.setText(findTestObject('Navigations/Navigation - Company Pay Rates/input_loginfmt'), 
    //'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Navigations/Navigation - Company Pay Rates/input_idSIButton9'))

//WebUI.click(findTestObject('Navigations/Navigation - Company Pay Rates/span_Your Pay  Benefits'))

//WebUI.click(findTestObject('Navigations/Navigation - Company Pay Rates/a_Company Rates'))

WebUI.delay(3)

WebUI.click(findTestObject('Navigations/Navigation - Company Pay Rates/a_Company Pay Rates'))

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Company Pay Rates/Search Company Pay Rates'))