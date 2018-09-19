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

//WebUI.navigateToUrl('https://login.microsoftonline.com/7ef41b2d-b0df-48c8-8b0c-c6497fdf26ac/oauth2/authorize?client_id=f6722b0a-c5be-4f78-99b5-b3c01917fbfe&response_mode=form_post&response_type=code%20id_token&scope=openid%20profile&state=OpenIdConnect.AuthenticationProperties%3D2o9R_gEnBaTDcumGlDFZ8f73rk5BriOg9jKMNrH6EOt_DiFxmiI4Qav7t8_jqvZJaaz1dx5JRXFREO-e2kInHm4CWXmJ7V6d-eF2L9CkhC2rd14LDxGEZqeUvzVmusLEoDyrruBG-iRykUJXGggPxA&nonce=636667083063702258.N2JkNTY3OWEtMDBkOS00OWZhLTgxNWQtMzkyYjBmZjc4NWVkNWM1OGY2NWUtZWYzZi00M2Y4LWIxNmItNDgzOGM0OWNlYjYy&x-client-SKU=ID_NET451&x-client-ver=5.2.1.0')

//WebUI.setText(findTestObject('Navigations/Navigation - Cost Code/input_loginfmt'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Navigations/Navigation - Cost Code/input_idSIButton9'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('Navigations/Navigation - Cost Code/button_Your Setup'))

WebUI.delay(2)

WebUI.mouseOver(findTestObject('Navigations/Navigation - Cost Code/a_Your Payroll'))

WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Cost Code/a_Cost Code'))

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Cost Code/input'))