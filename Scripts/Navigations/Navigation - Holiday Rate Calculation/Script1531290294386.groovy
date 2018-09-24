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
//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3Dkvu0923dgKqTO1zooUpoLHOsRnnpBvasYfchtvv4EOz1i52ddosC-ltGvhMmF5foKsJSucAd5QzMjM41t84De0B4zlh1hdyuSLM2zLjkwmv7G4LV9avjO0JMjZl99nGANlelO7d7t75E4r0_OsrUjA&nonce=636668871088558118.NGRkYTVkMjEtOWIyZi00MjBhLTgyOGUtYjZkMDMwMGVjODZmMWM4ZmYyNjktYjczZi00OGE2LTg3ZmMtMDAyZTc5NTM2ZGU0&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')
//WebUI.setText(findTestObject('Navigations/Navigation - Holiday Rate Calculation/input_loginfmt'), 'Sanjeev@sdworx.com')
//WebUI.click(findTestObject('Navigations/Navigation - Holiday Rate Calculation/input_idSIButton9'))
WebUI.openBrowser('')

WebUI.navigateToUrl('https://implementationnp.sdworx.co.uk/Account/Login?ReturnUrl=%2F')

WebUI.setText(findTestObject('Navigations/Navigation - Holiday Rate Calculation/input_Email'), 
    'nadia.fareedun@sdworx.com')

WebUI.setText(findTestObject('Navigations/Navigation - Holiday Rate Calculation/input_Password'), 
    'Password5!')

WebUI.click(findTestObject('Navigations/Navigation - Holiday Rate Calculation/input_btn btn-default'))

WebUI.selectOptionByValue(findTestObject('Navigations/Navigation - Holiday Rate Calculation/select_Please SelectOrg Test 1'), 
    '5ff3bc22-7cb8-4dac-bddf-c14c8e62cdbb', true)
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Holiday Rate Calculation/a_Your Payroll Rules'))
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Holiday Rate Calculation/a_Holiday Rate Calculation'))
WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Holiday Rate Calculation/button_Save'))

