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
import org.openqa.selenium.Keys as Keys


//WebUI.openBrowser('')

//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3D1GIfesCyQX0ZAdhae8kCRlpnHIQVf3H1gws9uJehuhylwD48yosIV96V4h5OEmUAo90xCy9buxEQFlGycYMwzcSQZ9mdccLcYXVqDC1jn3rwLkJoKWLQVXaKP6gdTkURarE8yagcaI3Mh-GpZWIcFQ&nonce=636668983996826006.YjQzOWQ1MmYtMDcwNC00MDVhLWI4NzUtZTAwOGYyMDBlOGQ5YmNlNjA2YjItMDBlYy00MjlmLWFkMjItMzhhMTA3YTA4N2Zj&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')

//WebUI.setText(findTestObject('Navigations/Navigation - Derived Rates/input_loginfmt'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Navigations/Navigation - Derived Rates/input_idSIButton9'))

//WebUI.click(findTestObject('Navigations/Navigation - Derived Rates/button_Your Setup'))

//WebUI.click(findTestObject('Navigations/Navigation - Derived Rates/a_Payroll Rates'))

WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Derived Rates/a_Derived Rates'))

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Derived Rates/Search Drived Rates'))
