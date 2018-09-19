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

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Screens/Bank Indemnity/input_Username'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Screens/Bank Indemnity/div_Email Address'))
WebUI.setText(findTestObject('Screens/Bank Indemnity/input_Password'), '1234')

WebUI.click(findTestObject('Screens/Bank Indemnity/input_Submit'))

WebUI.click(findTestObject('Screens/Bank Indemnity/div_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Bank Indemnity/button_Your Setup'))

WebUI.click(findTestObject('Screens/Bank Indemnity/a_Your Pay  Benefits'))

WebUI.click(findTestObject('Screens/Bank Indemnity/a_Lookups'))

WebUI.click(findTestObject('Screens/Bank Indemnity/a_Bank Indemnity'))

WebUI.doubleClick(findTestObject('Screens/Bank Indemnity/a_Bank Indemnity Statement'))

WebUI.setText(findTestObject('Screens/Bank Indemnity/input_org.BankIndemnity (1)'), 'Testing')

WebUI.click(findTestObject('Screens/Bank Indemnity/button_Save'))

